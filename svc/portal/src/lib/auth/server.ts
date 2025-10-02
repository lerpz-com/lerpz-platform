import { query, redirect } from "@solidjs/router"
import { getRequestEvent } from "solid-js/web"
import { auth } from "."

export const querySession = query(async () => {
  "use server"
  const event = getRequestEvent()
  if (!event) return null

  const session = await auth.api.getSession({
    headers: event.request.headers
  })

  return session ?? null
}, "session")

export const requireSession = query(async () => {
  "use server"
  const session = await querySession()
  if (!session) throw redirect("/login")
  return session
}, "require-session")
