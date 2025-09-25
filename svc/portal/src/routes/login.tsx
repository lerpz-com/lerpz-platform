import { Button } from "~/component/ui/button"
import { signIn } from "~/lib/auth/client"

export default function Login() {
  return (
    <div>
      <Button onClick={signIn}>Login here!</Button>
    </div>
  )
}
