import { A } from "@solidjs/router";
import { Button } from "~/component/ui/button.tsx";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "~/component/ui/card.tsx";
import { signOut } from "~/lib/auth/client.ts";

export default function Index() {
  return (
    <main class="w-full h-full flex justify-center items-center">
      <Card class="max-w-1/3">
        <CardHeader>
          <CardTitle>Signed in user</CardTitle>
          <CardDescription>This shows the signed in user</CardDescription>
        </CardHeader>
        <CardContent>
          Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. In
          enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo.
          Vestibulum fringilla pede sit amet augue.
          <div>
            <A href="/dashboard" class="underline">
              Go to dashboard
            </A>
          </div>
        </CardContent>
        <CardFooter>
          <Button variant="secondary" onClick={() => signOut()}>
            Sign Out
          </Button>
        </CardFooter>
      </Card>
    </main>
  );
}
