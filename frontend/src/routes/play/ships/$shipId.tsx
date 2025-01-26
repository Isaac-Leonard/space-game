import { createFileRoute } from "@tanstack/react-router";
import { getShipDetails } from "../../../api/ships";

export const Route = createFileRoute("/play/ships/$shipId")({
  component: Ship,
  loader: async ({ context, params }) => {
    return await getShipDetails(Number(params.shipId), context.user!.token);
  },
});

function Ship() {
  const ship = Route.useLoaderData();
  return (
    <div>
      <h1>{ship.name}</h1>
      <div>
        Coords: {ship.x}, {ship.y}{" "}
      </div>
      <div>Mass: {ship.mass}</div>
      <div>Speed: {ship.speed}</div>
      <div>Volume: {ship.volume}</div>
    </div>
  );
}
