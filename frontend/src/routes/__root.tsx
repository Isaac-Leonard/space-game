import {
  createRootRouteWithContext,
  Link,
  Outlet,
} from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { Context, useIsLoggedIn } from "../context";

export const Route = createRootRouteWithContext<Context>()({
  component: () => {
    return (
      <>
        <div className="p-2 flex gap-2">
          <Link to="/" className="[&.active]:font-bold">
            Home
          </Link>
          <Link to="/about" className="[&.active]:font-bold">
            About
          </Link>
          <Link to="/login">Login</Link>
          <Link to="/register">Register</Link>
        </div>
        <Outlet />
        <TanStackRouterDevtools />
      </>
    );
  },
});
