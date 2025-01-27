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
      <div>Mass: {ship.mass}</div>
      <div>Speed: {ship.speed}</div>
      <div>Volume: {ship.volume}</div>
    </div>
  );
}
