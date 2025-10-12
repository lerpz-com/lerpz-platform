"use server";

import { z } from "zod";
import process from "node:process";

export const envSchema = z.object({
  BETTER_AUTH_SECRET: z.string(),
  BETTER_AUTH_URL: z.url(),
  ENTRA_ID_TENANT_ID: z.string(),
  ENTRA_ID_CLIENT_ID: z.string(),
  ENTRA_ID_CLIENT_SECRET: z.string(),
  DATABASE_URL: z.string(),
});

const parsed = envSchema.parse(process.env);

export const env = parsed;
