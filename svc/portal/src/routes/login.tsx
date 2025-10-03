import { A } from "@solidjs/router"
import { Button } from "~/component/ui/button"
import { signIn } from "~/lib/auth/client"

export default function Login() {
  return (
    <div class="h-full flex flex-col items-center justify-center">
      <Button class="mb-4" onClick={signIn}>
        Login
      </Button>
      <A class="underline" href="/">
        Go to homepage
      </A>
    </div>
  )
}
