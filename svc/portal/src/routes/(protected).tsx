import { ParentProps } from "solid-js"
import { AuthContextProvider } from "~/component/auth-context"

export default function ProtectedLayout({ children }: ParentProps) {
  return <AuthContextProvider>{children}</AuthContextProvider>
}
