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
  return (
    <Card class="max-w-1/3">
      <CardHeader>
        <CardTitle>Signed in user</CardTitle>
        <CardDescription>This shows the signed in user</CardDescription>
      </CardHeader>
      <CardContent>
        Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. In enim
        justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Vestibulum
        fringilla pede sit amet augue.
      </CardContent>
      <CardFooter>
        <Button onClick={() => signOut()}>Sign Out</Button>
      </CardFooter>
    </Card>
  )
}

export default Home
