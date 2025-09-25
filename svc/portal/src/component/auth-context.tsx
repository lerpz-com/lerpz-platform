import { AccessorWithLatest, createAsync } from "@solidjs/router"
import { createContext, ParentProps, useContext } from "solid-js"
import { querySession } from "~/lib/auth/server"

type AuthenticatedSessionData = {
  user: {
    id: string
    createdAt: Date
    updatedAt: Date
    email: string
    emailVerified: boolean
    name: string
    image?: string | null | undefined
  }
  session: {
    id: string
    createdAt: Date
    updatedAt: Date
    userId: string
    expiresAt: Date
    token: string
    ipAddress?: string | null | undefined
    userAgent?: string | null | undefined
  }
}

const AuthContext = createContext<{
  session: AccessorWithLatest<AuthenticatedSessionData | null | undefined>
}>()

export const useAuth = () => {
  const context = useContext(AuthContext)
  console.log(context)
  if (!context) {
    throw new Error("useAuth must be used within an AuthContextProvider")
  }
  return context
}

export const AuthContextProvider = (props: ParentProps) => {
  const session = createAsync(() => querySession(), {
    deferStream: true
  })

  return (
    <AuthContext.Provider value={{ session }}>
      {props.children}
    </AuthContext.Provider>
  )
}
