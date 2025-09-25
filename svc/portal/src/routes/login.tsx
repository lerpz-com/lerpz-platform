import { A } from "@solidjs/router"
import { Button } from "~/component/ui/button"
import { signIn } from "~/lib/auth/client"

export default function Login() {
  return (
    <>
      <Button onClick={signIn}>Login</Button>
      <A href="/">Go to homepage</A>
    </>
  )
}
