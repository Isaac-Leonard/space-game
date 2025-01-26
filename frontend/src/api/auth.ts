import * as Z from "zod";
import { post } from "./utils";

type NewUser = { name: string; email: string; password: string };

type LoginParams = { email: string; password: string };

type ForgotPasswordParams = { email: string };

export const loginResponse = Z.object({
  token: Z.string(),
  pid: Z.string(),
  name: Z.string(),
  is_verified: Z.boolean(),
  role: Z.string(),
});

const registerResponse = Z.unknown();

const resetPasswordResponse = Z.object({});

export const registerUser = async (user: NewUser) => {
  return await post("/api/auth/register", user, registerResponse);
};

export const login = async (user: LoginParams) => {
  return await post("/api/auth/login", user, loginResponse);
};

export type User = Awaited<ReturnType<typeof login>>;

export const resetPassword = async (resetParams: ForgotPasswordParams) => {
  return await post("/api/auth/forgot", resetParams, resetPasswordResponse);
};
