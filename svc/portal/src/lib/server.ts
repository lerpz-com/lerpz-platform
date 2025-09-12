import { useSession } from "vinxi/http";
import { redirect } from "@solidjs/router";

export interface Session {
  id: number;
  email: string;
}

export const getSession = () =>
  useSession<Session>({
    password: process.env.SESSION_SECRET!
  });

export const createSession = async (user: Session, redirectTo?: string) => {
  const validDest = redirectTo?.[0] === "/" && redirectTo[1] !== "/";
  const session = await getSession();
  await session.update(user);
  return redirect(validDest ? redirectTo : "/");
};
