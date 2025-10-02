import { betterAuth } from "better-auth"
import { Pool } from "pg"
import { env } from "~/server/env"

export const auth = betterAuth({
  database: new Pool({
    connectionString: env.DATABASE_URL
  }),
  // plugins: [sso()],
  account: {
    accountLinking: {
      enabled: false
    }
  },
  socialProviders: {
    microsoft: {
      clientId: env.ENTRA_ID_CLIENT_ID as string,
      clientSecret: env.ENTRA_ID_CLIENT_SECRET as string,
      tenantId: env.ENTRA_ID_TENANT_ID as string,
      authority: "https://login.microsoftonline.com",
      scopes: ["openid", "profile", "email", "User.Read"],
      prompt: "select_account" // Forces account selection
    }
  }
})

export type Session = typeof auth.$Infer.Session

export default auth
