import * as Z from "zod";
import { authenticatedGet } from "./utils";

const ShipValidator = Z.object({
  id: Z.number(),
  name: Z.string(),
  x: Z.number(),
  y: Z.number(),
  z: Z.number(),
  speed: Z.number(),
  mass: Z.number(),
  type: Z.string(),
  volume: Z.number(),
});

export type Ship = Z.TypeOf<typeof ShipValidator>;

export const getShips = async (token: string): Promise<Ship[]> => {
  console.log("Called getShips");
  const ships = await authenticatedGet(
    "/api/spacecrafts",
    token,
    ShipValidator.array()
  );
  console.log(ships);
  return ships;
};

export const getShipDetails = async (
  shipId: number,
  token: string
): Promise<Ship> => {
  const ship = await authenticatedGet(
    `/api/spacecrafts/${shipId}`,
    token,
    ShipValidator
  );
  return ship;
};
