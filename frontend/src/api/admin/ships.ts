import * as Z from "zod";
import { authenticatedGet } from "../utils";

const AdminShips = Z.object({
  name: Z.string(),
  id: Z.number(),
  user: Z.string(),
  user_name: Z.string(),
  type: Z.string(),
});

export const getShips = async (token: string) => {
  return await authenticatedGet(
    "/api/admin/spacecrafts",
    token,
    AdminShips.array()
  );
};
