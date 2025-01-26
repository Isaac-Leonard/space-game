import { authenticatedGetFactory } from "./utils";
import * as Z from "zod";
const SystemValidator = Z.object({
  x: Z.number(),
  y: Z.number(),
  mass: Z.number(),
  radius: Z.number(),
  type: Z.string(),
});

export type System = Z.TypeOf<typeof SystemValidator>;

export const getNearbySystems = authenticatedGetFactory(
  "/api/systems/nearby",
  SystemValidator.array()
);

export const SystemDetailsValidator = Z.object({
  id: Z.number(),
  x: Z.number(),
  y: Z.number(),
  radius: Z.number(),
  composition: Z.record(Z.string(), Z.number()),
  planets: Z.number(),
});

export type SystemDetails = Z.TypeOf<typeof SystemDetailsValidator>;

export const getCurrentSystemDetails = authenticatedGetFactory(
  "/api/systems/current_system",
  SystemDetailsValidator
);
