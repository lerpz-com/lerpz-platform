import { ParentProps } from "solid-js"
import { AppShell } from "~/component/app-shell"

export default function Dashboard({ children }: ParentProps) {
  return <AppShell>{children}</AppShell>
}
