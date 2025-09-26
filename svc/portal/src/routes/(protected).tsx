import { useNavigate } from "@solidjs/router"
import { createEffect, ParentProps, Show } from "solid-js"
import { useAuth } from "~/component/auth-context"

export default function ProtectedLayout({ children }: ParentProps) {
  const navigate = useNavigate()
  const { session } = useAuth()

  createEffect(() => {
    if (!session()) {
      navigate("/login")
    }
  })

  return <Show when={!!session()}>{children}</Show>
}
