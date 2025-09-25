import { action, query, redirect } from "@solidjs/router"
import { createAuthClient } from "better-auth/solid"

export const authClient = createAuthClient({
  // plugins: [ssoClient()]
})

export const signIn = async () => {
  await authClient.signIn.social({
    provider: "microsoft"
  })
}

export const {
  signOut,
  useSession,
  getSession,
  getAccessToken,
  revokeSession
} = authClient

export const querySession = query(async () => {
  "use server"
  const { data } = await getSession()
  if (data) return data
  return redirect("/login")
}, "session")

export const logout = action(async () => {
  "use server"
  const session = await getSession()
  if (!session.data) return null
  revokeSession({
    token: session.data.session.token
  })
  throw redirect("/login", { revalidate: "session" })
})
