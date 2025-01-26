import { createFileRoute } from "@tanstack/react-router";
import { useActionState, useState } from "react";
import { registerUser } from "../api/auth";

export type RegisterProps = { onRegister: () => void };

const Register = () => {
  const [registered, setRegistered] = useState(false);
  const [error, action, loading] = useActionState(
    async (_err: unknown, e: FormData) => {
      const password = e.get("password") as string;
      const confirmPassword = e.get("confirmPassword") as string;

      if (password !== confirmPassword) {
        return { err: "passwords do not match" };
      }

      const data = {
        name: e.get("name") as string,
        email: e.get("email")! as string,
        password,
      };
      try {
        const user = await registerUser(data);
        setRegistered(true);
        return user;
      } catch (e) {
        return e;
      }
    },
    null
  );

  return registered ? (
    <div role="alert">
      A verification link has been sent to your email, please click that link
      then login
    </div>
  ) : error ? (
    JSON.stringify(error)
  ) : (
    <form action={action}>
      <label>
        Username:<input name="name"></input>
      </label>
      <label>
        Email:<input type="email" name="email"></input>
      </label>
      <label>
        Password:<input type="password" name="password"></input>
      </label>
      <label>
        Confirm password:
        <input type="password" name="confirmPassword"></input>
      </label>
      <input type="submit" disabled={loading}></input>
    </form>
  );
};

export const Route = createFileRoute("/register")({ component: Register });
