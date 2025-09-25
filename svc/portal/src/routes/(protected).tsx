import { ParentProps, Show } from "solid-js"
import { useAuth } from "~/component/auth-context"

export default function ProtectedLayout({ children }: ParentProps) {
  const { session } = useAuth()

  return <Show when={!!session()}>{children}</Show>
}
