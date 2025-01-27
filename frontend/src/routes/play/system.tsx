import { createFileRoute } from "@tanstack/react-router";
import { client, throwError } from "../../client";

export const Route = createFileRoute("/play/system")({
  component: System,
  loader: async () => throwError(await client.api.getSystemsCurrentSystem()),
});

function System() {
  const system = Route.useLoaderData();
  console.log(system);
  return (
    <div>
      <h1>System details: {system.id}</h1>
      <div>
        <h2>Star</h2>
        <table>
          <thead>
            <tr>
              <th>Element</th>
              <th>Mass (Gt)</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(system.composition).map(([element, mass]) => (
              <tr>
                <td>{element}</td>
                <td>{(mass / 1e12).toPrecision(3)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <h2>Planets</h2>
      <ul>
        {system.planets.map((planet) => (
          <li key={planet.id}>{planet.type}</li>
        ))}
      </ul>
    </div>
  );
}
