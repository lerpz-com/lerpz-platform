import OAuth from "start-oauth";
import type { Configuration } from "start-oauth/src/types";
import { oauthSignIn } from "~/lib/server";

const config: Configuration = {
  password: process.env.SESSION_SECRET!,
  microsoft: {
    id: process.env.MICROSOFT_CLIENT_ID!,
    secret: process.env.MICROSOFT_CLIENT_SECRET!
  },
  handler: async ({ email }, redirectTo) => oauthSignIn(email, redirectTo),
};

export const GET = OAuth(config);
