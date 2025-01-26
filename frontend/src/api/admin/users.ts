import * as Z from "zod";
import { authenticatedGet } from "../utils";

const User = Z.object({
  pid: Z.string(),
  name: Z.string(),
  role: Z.string(),
});

export const getUsers = async (token: string) => {
  return await authenticatedGet("/api/admin/users", token, User.array());
};

const UserDetails = Z.object({
  pid: Z.string(),
  name: Z.string(),
  role: Z.string(),
  ships: Z.object({
    id: Z.number(),
    name: Z.string(),
    type: Z.string(),
  }).array(),
});

export const getUserDetails = async (token: string, user: string) => {
  return await authenticatedGet(`/api/admin/users/${user}`, token, UserDetails);
};
