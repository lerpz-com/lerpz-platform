import { betterAuth } from "better-auth"

export const auth = betterAuth({
  socialProviders: {
    microsoft: {
      clientId: process.env.ENTRA_ID_CLIENT_ID as string,
      clientSecret: process.env.ENTRA_ID_CLIENT_SECRET as string,
      tenantId: process.env.ENTRA_ID_TENANT_ID as string,
      authority: "https://login.microsoftonline.com",
      prompt: "select_account" // Forces account selection
    }
  }
})
