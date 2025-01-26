import { StrictMode, useState } from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";

// Import the generated route tree
import { routeTree } from "./routeTree.gen";
import { AppContext } from "./context";
import { User } from "./api/auth";

// Create a new router instance
const router = createRouter({
  routeTree,
  context: { user: null, setUser: () => {} },
});

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const App = () => {
  const [user, setUser] = useState<User | null>(null);
  const context = { user, setUser };
  return (
    <StrictMode>
      <AppContext value={context}>
        <RouterProvider router={router} context={context} />
      </AppContext>
    </StrictMode>
  );
};

// Render the app
const rootElement = document.getElementById("root");
if (rootElement) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(<App></App>);
}
