import { createFileRoute } from "@tanstack/react-router";
import { getCurrentSystemDetails } from "../../api/nearby";

export const Route = createFileRoute("/play/system")({
  component: System,
  loader: ({ context }) => getCurrentSystemDetails(context.user!.token),
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
      <div>There are {system.planets} in this system</div>
    </div>
  );
}
