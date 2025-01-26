import { createFileRoute, Link } from "@tanstack/react-router";
import { admin } from "../../../api/admin";

export const Route = createFileRoute("/admin/users/$userId")({
  component: User,
  loader: async ({ context, params }) => {
    console.log("Fetching user details");
    const user = await admin.getUserDetails(context.user!.token, params.userId);
    console.log("Fetched user details");
    return user;
  },
});

function User() {
  console.log("Rendering user details");
  const user = Route.useLoaderData();
  return (
    <div>
      <h1>
        Details for {user.role} {user.name}
      </h1>
      <h2>Owned Ships</h2>
      <ul>
        {user.ships.map(({ id, name, type }) => (
          <li key={id}>
            <Link to=".">
              {name}: {type}
            </Link>
          </li>
        ))}
      </ul>
      <button>Add ship to {user.name}</button>
    </div>
  );
}
