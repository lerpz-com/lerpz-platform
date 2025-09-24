import { createAsync, query } from "@solidjs/router"
import { createContext, ParentProps } from "solid-js"
import { signIn, useSession } from "~/lib/auth/client"

const AuthContext = createContext<undefined>()

const getUser = query(async function () {
  "use server"
  const session = useSession()
  const data = session().data
  if (!data) {
    throw signIn()
  }
  return data
}, "getUser")

export const AuthContextProvider = (props: ParentProps) => {
  createAsync(() => getUser())

  return (
    <AuthContext.Provider value={undefined}>
      {props.children}
    </AuthContext.Provider>
  )
}
