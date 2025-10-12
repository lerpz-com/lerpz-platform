import type { ParentProps } from "solid-js";
import { AppShell } from "~/component/app-shell/index.tsx";

export default function Dashboard({ children }: ParentProps) {
  return <AppShell>{children}</AppShell>;
}
