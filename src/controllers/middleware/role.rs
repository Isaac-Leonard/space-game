use aide::{
    generate::GenContext, openapi::Operation, transform::TransformOperation, OperationInput,
};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use loco_rs::{app::AppContext, controller::middleware::auth, Error};

use crate::models::users;

pub struct AdminUser(pub users::Model);

impl OperationInput for AdminUser {
    fn operation_input(ctx: &mut GenContext, operation: &mut Operation) {
        let t = TransformOperation::new(operation);
        let _ = t.security_requirement("bearer_token");
    }
}

impl<S> FromRequestParts<S> for AdminUser
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = auth::JWTWithUser::<users::Model>::from_request_parts(parts, &state).await?;
        tracing::debug!("Checking admin role for {}", user.user.name);
        if user.user.role != "admin" {
            return Err(Error::Unauthorized(
                "Admin role required to access this".to_string(),
            ));
        };
        return Ok(AdminUser(user.user));
    }
}
