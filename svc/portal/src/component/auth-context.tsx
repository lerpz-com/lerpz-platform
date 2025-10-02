import { AccessorWithLatest, createAsync } from "@solidjs/router"
import { createContext, ParentProps, useContext } from "solid-js"
import { Session } from "~/lib/auth"
import { querySession } from "~/lib/auth/server"

const AuthContext = createContext<{
  session: AccessorWithLatest<Session | null | undefined>
}>(undefined)

export const useAuth = () => {
  const context = useContext(AuthContext)
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
