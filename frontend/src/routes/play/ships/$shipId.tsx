import { createFileRoute } from "@tanstack/react-router";
import { client, throwError } from "../../../client";

export const Route = createFileRoute("/play/ships/$shipId")({
  component: Ship,
  loader: async ({ params }) =>
    throwError(
      await client.api.getSpacecraftsId({
        parameters: { path: { id: Number(params.shipId) } },
      })
    ),
});

function Ship() {
  const ship = Route.useLoaderData();
  return (
    <div>
      <h1>{ship.name}</h1>
      <div>
        Coords: {ship.coordinates.x}, {ship.coordinates.y}{" "}
      </div>
      <div>Mass: {ship.mass} kg</div>
      <div>Speed: {ship.speed} m/s</div>
      <div>
        Holdspace: {ship.volume} m<sup>3</sup>
      </div>
    </div>
  );
}
