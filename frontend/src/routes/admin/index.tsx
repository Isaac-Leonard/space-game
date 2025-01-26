import { createFileRoute, Link, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/admin/")({
  component: Index,
  beforeLoad: ({ context }) => {
    if (context.user?.role !== "admin") {
      redirect({ to: "/login" });
    }
  },
});

function Index() {
  return (
    <div>
      <h1>Admin Pannel</h1>
      <div>
        <Link to="/admin/users">Users</Link>
        <Link to="/admin/ships">Ships</Link>
      </div>
    </div>
  );
}
