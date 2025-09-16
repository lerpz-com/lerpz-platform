import { betterAuth } from "better-auth"
import Database from "better-sqlite3"
import { env } from "~/lib/env"

export const auth = betterAuth({
  database: new Database("./data/auth.db"),
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
