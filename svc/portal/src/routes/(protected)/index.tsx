import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardTitle
} from "~/component/ui/card"
import { useSession } from "~/lib/auth/client"

const Home = () => {
  const session = useSession()

  return (
    <Card>
      <CardTitle>Signed in user</CardTitle>
      <CardDescription>This shows the signed in user</CardDescription>
      <CardContent>{session().data?.user.name || "Unknown User"}</CardContent>
      <CardFooter>LERPZ.COM</CardFooter>
    </Card>
  )
}

export default Home
