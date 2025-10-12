import { type RouteDefinition, useNavigate } from "@solidjs/router";
import { createEffect, type ParentProps, Show } from "solid-js";
import { useAuth } from "~/component/auth-context.tsx";
import { requireSession } from "~/lib/auth/server.ts";

export const route: RouteDefinition = {
  preload: () => requireSession(),
};

export default function ProtectedLayout({ children }: ParentProps) {
  const navigate = useNavigate();
  const { session } = useAuth();

  createEffect(() => {
    if (!session()) {
      navigate("/login");
    }
  });

  return <Show when={session()}>{children}</Show>;
}
