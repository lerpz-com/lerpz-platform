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
