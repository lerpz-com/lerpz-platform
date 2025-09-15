import { createAuthClient } from "better-auth/solid"

export const authClient = createAuthClient({
  baseURL: "https://localhost:3000"
})

export const { signIn, signUp, useSession } = createAuthClient()
