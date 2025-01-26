import { createFileRoute, Link } from "@tanstack/react-router";
import { admin } from "../../api/admin";

export const Route = createFileRoute("/admin/ships")({
  component: Ships,
  loader: async ({ context }) => {
    return await admin.getShips(context.user!.token);
  },
});

function Ships() {
  const ships = Route.useLoaderData();
  return (
    <div>
      <h1>Ships Admin Pannel</h1>
      <ul>
        {ships.map(({ id, name, user, user_name, type }) => (
          <li>
            <Link
              to="."
              // to="/admin/ships/$shipId"
            >
              {name}: {type}
            </Link>
            Owned by
            <Link
              to="."
              // to="/admin/users/$userId"
            >
              {user_name}
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}
