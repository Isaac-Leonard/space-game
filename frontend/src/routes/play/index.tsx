import {
  createFileRoute,
  Link,
  redirect,
  useNavigate,
} from "@tanstack/react-router";
import { useAppContext } from "../../context";

const Play = () => {
  const context = useAppContext();
  const navigate = useNavigate();
  const logout = () => {
    context.setUser(null);
    navigate({ to: "/" });
  };
  return (
    <div>
      <h1>Main menu</h1>
      <Link to="/play/system">Current System</Link>
      <Link to="/play/local_area">Nearby Systems</Link>
      <Link to="/play/ships">Ships</Link>
      <Link to="/play/relay">Galactic relay</Link>
      <Link to="/play/account">Account</Link>
      <button onClick={logout}>Logout</button>
    </div>
  );
};

export const Route = createFileRoute("/play/")({
  component: Play,
  beforeLoad: ({ context }) => {
    const isLoggedIn = context.user !== null;
    if (!isLoggedIn) {
      throw redirect({ to: "/login" });
    }
  },
});
