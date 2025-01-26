import { createFileRoute, Link } from "@tanstack/react-router";
import { admin } from "../../../api/admin";

export const Route = createFileRoute("/admin/users/")({
  component: Users,
  loader: async ({ context }) => {
    return await admin.getUsers(context.user!.token);
  },
});

function Users() {
  const users = Route.useLoaderData();
  return (
    <div>
      <h1>Users Admin Pannel</h1>
      <ul>
        {users.map(({ pid, name, role }) => (
          <li key={pid}>
            <Link to="/admin/users/$userId" params={{ userId: pid }}>
              {name}: {role}
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}
