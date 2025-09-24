"use server"

import { treeifyError, z } from "zod"

export const envSchema = z.object({
  BETTER_AUTH_SECRET: z.string(),
  BETTER_AUTH_URL: z.url(),
  ENTRA_ID_TENANT_ID: z.string(),
  ENTRA_ID_CLIENT_ID: z.string(),
  ENTRA_ID_CLIENT_SECRET: z.string()
})

const parsed = envSchema.safeParse(process.env)

if (parsed.error) {
  console.error("❌ Invalid environment variables:", treeifyError(parsed.error))
  throw new Error("Invalid environment variables")
}

export const env = parsed.data
