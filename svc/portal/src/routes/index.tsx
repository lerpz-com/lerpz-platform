import { Show, type VoidComponent } from "solid-js"
import { Button } from "~/component/ui/button"
import { signIn, signOut, useSession } from "~/lib/auth-client"

const Home: VoidComponent = () => {
  const session = useSession()

  return (
    <main class="flex min-h-screen flex-col items-center justify-center">
      <Button onClick={() => signOut()}>Sign out</Button>
      <Show
        when={session().data}
        fallback={
          <Button onClick={() => signIn.social({ provider: "microsoft" })}>
            Sign in
          </Button>
        }
      >
        {session().data?.user.email}
      </Show>
    </main>
  )
}

export default Home
