import type { AccessorWithLatest } from "@solidjs/router";
import { createAsync } from "@solidjs/router";
import type { ParentProps } from "solid-js";
import { createContext, useContext } from "solid-js";
import type { Session } from "~/lib/auth/index.ts";
import { querySession } from "~/lib/auth/server.ts";

const AuthContext = createContext<
  | {
      session: AccessorWithLatest<Session | null | undefined>;
    }
  | undefined
>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};

export const AuthProvider = ({ children }: ParentProps) => {
  const session = createAsync(() => querySession(), {
    deferStream: true,
  });

  return (
    <AuthContext.Provider value={{ session }}>{children}</AuthContext.Provider>
  );
};
