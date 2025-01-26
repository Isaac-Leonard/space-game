use crate::{
    mailers::auth::AuthMailer,
    models::{
        spacecrafts::{self, NewShip},
        users::{self, LoginParams, RegisterParams},
    },
    views::auth::{CurrentResponse, LoginResponse},
};
use aide::axum::{routing::get_with, IntoApiResponse};
use axum::{debug_handler, http::StatusCode, response::Redirect};
use loco_rs::prelude::*;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub static EMAIL_DOMAIN_RE: OnceLock<Regex> = OnceLock::new();

fn get_allow_email_domain_re() -> &'static Regex {
    EMAIL_DOMAIN_RE.get_or_init(|| {
        Regex::new(r"@example\.com$|@gmail\.com$").expect("Failed to compile regex")
    })
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct MagicLinkParams {
    pub email: String,
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
// #[debug_handler]
async fn register(State(ctx): State<AppContext>, Json(params): Json<RegisterParams>) -> Json<()> {
    eprintln!("Register called");
    let transaction = ctx.db.begin().await.unwrap();
    let res = users::Model::create_with_password(&transaction, &params).await;
    eprintln!("User created");

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return Json(());
        }
    };

    let new_craft = NewShip {
        name: "Ship 1",
        user: user.id,
        mass: 10000.0,
        speed: 5.0,
        r#type: "Shuttle".to_string(),
    };
    let craft = spacecrafts::Model::create_ship_for_user(&transaction, &new_craft).await;
    let _ = craft
        .map_err(|e| e.to_string())
        .inspect_err(|e| tracing::debug!(e, "error for new user spacecraft"))
        .inspect(|craft| {
            let craft = serde_json::to_string(craft).unwrap();
            tracing::debug!(craft, "new user spacecraft")
        });

    let user = user
        .into_active_model()
        .set_email_verification_sent(&transaction)
        .await
        .unwrap();

    AuthMailer::send_welcome(&ctx, &user).await.unwrap();
    transaction.commit().await.unwrap();
    Json(())
}

#[derive(Debug, Clone, PartialEq, Deserialize, JsonSchema)]
pub struct VerificationToken {
    pub token: String,
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Path(token): Path<VerificationToken>,
) -> impl IntoApiResponse {
    let user = users::Model::find_by_verification_token(&ctx.db, &token.token)
        .await
        .unwrap();

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await.unwrap();
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }
    Redirect::to("/play")
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(State(ctx): State<AppContext>, Json(params): Json<ForgotParams>) -> Json<()> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return Json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await
        .unwrap();

    AuthMailer::forgot_password(&ctx, &user).await.unwrap();

    Json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Json<()> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return Json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await
        .unwrap();

    Json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(
    State(ctx): State<AppContext>,
    Json(params): Json<LoginParams>,
) -> Json<LoginResponse> {
    let user = users::Model::find_by_email(&ctx.db, &params.email)
        .await
        .unwrap();

    let valid = user.verify_password(&params.password);

    if !valid {
        panic!("unauthorized!");
    }

    let jwt_secret = ctx.config.get_jwt_config().unwrap();

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))
        .unwrap();

    Json(LoginResponse::new(&user, &token))
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Json<CurrentResponse> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid)
        .await
        .unwrap();
    Json(CurrentResponse::new(&user))
}

/// Magic link authentication provides a secure and passwordless way to log in to the application.
///
/// # Flow
/// 1. **Request a Magic Link**:  
///    A registered user sends a POST request to `/magic-link` with their email.  
///    If the email exists, a short-lived, one-time-use token is generated and sent to the user's email.  
///    For security and to avoid exposing whether an email exists, the response always returns 200, even if the email is invalid.
///
/// 2. **Click the Magic Link**:  
///    The user clicks the link (/magic-link/{token}), which validates the token and its expiration.  
///    If valid, the server generates a JWT and responds with a [`LoginResponse`].  
///    If invalid or expired, an unauthorized response is returned.
///
/// This flow enhances security by avoiding traditional passwords and providing a seamless login experience.
async fn magic_link(
    State(ctx): State<AppContext>,
    Json(params): Json<MagicLinkParams>,
) -> Json<()> {
    let email_regex = get_allow_email_domain_re();
    if !email_regex.is_match(&params.email) {
        tracing::debug!(
            email = params.email,
            "The provided email is invalid or does not match the allowed domains"
        );
        panic!("invalid request");
    }

    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::debug!(email = params.email, "user not found by email");
        return Json(());
    };

    let user = user
        .into_active_model()
        .create_magic_link(&ctx.db)
        .await
        .unwrap();
    AuthMailer::send_magic_link(&ctx, &user).await.unwrap();

    Json(())
}

#[derive(Debug, Clone, PartialEq, Deserialize, JsonSchema)]
pub struct MagicLinkToken {
    pub token: String,
}

/// Verifies a magic link token and authenticates the user.
async fn magic_link_verify(
    State(ctx): State<AppContext>,
    Path(token): Path<MagicLinkToken>,
) -> Json<LoginResponse> {
    let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        panic!("unauthorized!");
    };

    let user = user
        .into_active_model()
        .clear_magic_link(&ctx.db)
        .await
        .unwrap();

    let jwt_secret = ctx.config.get_jwt_config().unwrap();

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))
        .unwrap();

    Json(LoginResponse::new(&user, &token))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth")
        .add("/register", post(register))
        .add(
            "/verify/{token}",
            get_with(verify, |op| {
                op.response::<{ StatusCode::PERMANENT_REDIRECT.as_u16() }, Json<()>>()
            }),
        )
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/current", get(current))
        .add("/magic-link", post(magic_link))
        .add("/magic-link/{token}", get(magic_link_verify))
}
