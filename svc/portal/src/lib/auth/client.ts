import { createAuthClient } from "better-auth/solid"
import { ssoClient } from "@better-auth/sso/client"

export const authClient = createAuthClient({
  plugins: [ssoClient()]
})
