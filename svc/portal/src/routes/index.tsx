import { Show, type VoidComponent } from "solid-js"
import { signIn, signOut, useSession } from "~/lib/auth-client"

const Home: VoidComponent = () => {
  const session = useSession()

  return (
    <main class="flex min-h-screen flex-col items-center justify-center">
      <div class="container flex flex-col items-center justify-center gap-12 px-4 py-16 ">
        <button onClick={() => signOut()}>Sign out</button>
        <Show
          when={session().data}
          fallback={
            <button onClick={() => signIn.social({ provider: "microsoft" })}>
              Sign in
            </button>
          }
        >
          {session().data?.user.email}
        </Show>
      </div>
    </main>
  )
}

export default Home
