import { betterAuth } from "better-auth"
import { env } from "~/server/env"

export const auth = betterAuth({
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
