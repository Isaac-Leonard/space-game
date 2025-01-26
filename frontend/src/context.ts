import { createContext, useContext } from "react";
import { User } from "./api/auth";

export type Context = {
  user: User | null;
  setUser: (user: User | null) => void;
};

export const AppContext = createContext<Context>({
  user: null,
  setUser: () => {},
});

export const useIsLoggedIn = (): boolean => {
  return useContext(AppContext).user !== null;
};

export const useAppContext = () => useContext(AppContext);
