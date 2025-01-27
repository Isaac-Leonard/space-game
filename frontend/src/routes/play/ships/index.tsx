import { createFileRoute, Link } from "@tanstack/react-router";
import { client, throwError } from "../../../client";

function Ships() {
  const ships = Route.useLoaderData();
  return (
    <div>
      <h1>Owned ships</h1>
      <ul>
        {ships.map(({ name, id, type }) => (
          <li>
            <Link
              key={id}
              to="/play/ships/$shipId"
              params={{ shipId: id.toString() }}
            >
              {name}: {type}
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}

export const Route = createFileRoute("/play/ships/")({
  component: Ships,
  loader: async () => throwError(await client.api.getSpacecrafts()),
});
