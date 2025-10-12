import { toSolidStartHandler } from "better-auth/solid-start";
import { auth } from "~/lib/auth/index.ts";

export const { GET, POST } = toSolidStartHandler(auth);
