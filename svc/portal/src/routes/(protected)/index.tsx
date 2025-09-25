import { useAuth } from "~/component/auth-context"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from "~/component/ui/card"

const Home = () => {
  const { session } = useAuth()

  return (
    <Card>
      <CardHeader>
        <CardTitle>Signed in user</CardTitle>
        <CardDescription>This shows the signed in user</CardDescription>
      </CardHeader>
      <CardContent>{session()?.user.email || "Unknown user"}</CardContent>
      <CardFooter>LERPZ.COM</CardFooter>
    </Card>
  )
}

export default Home
