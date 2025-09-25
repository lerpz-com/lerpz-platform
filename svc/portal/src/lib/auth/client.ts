import { revalidate } from "@solidjs/router"
import { createAuthClient } from "better-auth/solid"
import { querySession } from "./server"

export const authClient = createAuthClient({
  // plugins: [ssoClient()]
})

export const signIn = async () => {
  await authClient.signIn.social({
    provider: "microsoft"
  })
}

export const signOut = async () => {
  await authClient.signOut()
  revalidate(querySession.key)
}

export const { useSession } = authClient
