import type { ParentProps } from "solid-js";
import { AppShell } from "~/component/app-shell";

export default function Visuals({ children }: ParentProps) {
  return <AppShell>{children}</AppShell>;
}
