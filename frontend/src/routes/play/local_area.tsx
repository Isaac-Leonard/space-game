import { client, throwError } from "../../client";
import { createFileRoute, Link } from "@tanstack/react-router";
import { useState } from "react";

export const Route = createFileRoute("/play/local_area")({
  component: NearbySystems,
  loader: async () => throwError(await client.api.getSystemsNearby()),
});

const LIGHTYEARFACTOR = 9.6e15;

function NearbySystems() {
  const systems = Route.useLoaderData();
  const [coordsOrDistance, setCoordsOrDistance] = useState(false);
  const dummyLocation = { x: 0, y: 0 };
  return (
    <div>
      <h1>Local space</h1>
      <div>
        At galactic coordinates x: {dummyLocation.x}, y: {dummyLocation.y}
      </div>
      <h2>Near by objects</h2>
      <button onClick={() => setCoordsOrDistance(!coordsOrDistance)}>
        View {coordsOrDistance ? "distance" : "coordinates"}
      </button>
      <ul>
        {systems.map(({ type, x, y }) => (
          <li>
            <Link to=".">
              {coordsOrDistance
                ? `${(x / LIGHTYEARFACTOR).toPrecision(3)}, ${(
                    y / LIGHTYEARFACTOR
                  ).toPrecision(3)}`
                : (Math.sqrt(x ** 2 + y ** 2) / LIGHTYEARFACTOR).toPrecision(3)}
              : {type}
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}
