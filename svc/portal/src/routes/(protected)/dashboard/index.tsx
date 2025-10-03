import { useAuth } from "~/component/auth-context"
import { Button } from "~/component/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from "~/component/ui/card"
import { signOut } from "~/lib/auth/client"

export default function Home() {
  const { session } = useAuth()

  return (
    <Card>
      <CardHeader>
        <CardTitle>Who Am I?</CardTitle>
        <CardDescription>This shows the signed in user</CardDescription>
      </CardHeader>
      <CardContent>Current email: {session()?.user?.email}</CardContent>
      <CardFooter>
        <Button onClick={() => signOut()}>Sign Out</Button>
      </CardFooter>
    </Card>
  )
}
