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

const Home = () => {
  const { session } = useAuth()

  return (
    <Card>
      <CardHeader>
        <CardTitle>Signed in user</CardTitle>
        <CardDescription>This shows the signed in user</CardDescription>
      </CardHeader>
      <CardContent>
        {session()?.user.email || "Unknown user"}
        <Button onClick={() => signOut()}>Sign Out</Button>
      </CardContent>
      <CardFooter>LERPZ.COM</CardFooter>
    </Card>
  )
}

export default Home
