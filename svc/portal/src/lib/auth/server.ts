import { query, redirect } from "@solidjs/router"
import { getRequestEvent } from "solid-js/web"
import { auth } from "."

export const querySession = query(async () => {
  "use server"
  const event = getRequestEvent()
  if (!event) throw redirect("/login")

  const session = await auth.api.getSession({
    headers: event.request.headers
  })
  console.log("Session:", session)
  if (!session) throw redirect("/login")

  return session
}, "session")
