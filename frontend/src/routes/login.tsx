import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useActionState } from "react";
import { login } from "../api/auth";
import { useAppContext } from "../context";
import { setAuthorisationToken } from "../client";

const Login = () => {
  const context = useAppContext();
  const navigate = useNavigate();
  const [error, action, loading] = useActionState(
    async (_err: null, e: FormData) => {
      const data = {
        email: e.get("email")! as string,
        password: e.get("password") as string,
      };
      const user = await login(data);
      context.setUser(user);
      setAuthorisationToken(user.token);
      if (user.role === "admin") {
        navigate({ to: "/admin" });
      } else {
        navigate({ to: "/play" });
      }
      return null;
    },
    null
  );

  return (
    <form action={action}>
      <label>
        Email<input type="email" name="email"></input>
      </label>
      <label>
        Password:<input type="password" name="password"></input>
      </label>
      <input type="submit" disabled={loading}></input>
    </form>
  );
};

export const Route = createFileRoute("/login")({ component: Login });
