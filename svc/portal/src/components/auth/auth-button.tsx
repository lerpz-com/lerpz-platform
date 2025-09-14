"use client"

import { useMsal, useAccount } from "@azure/msal-react"
import { loginRequest } from "@/lib/msalConfig"

export default function AuthButton() {
  const { instance, accounts } = useMsal()
  const account = useAccount(accounts[0] || {})

  const handleLogin = () => {
    instance.loginPopup(loginRequest).catch((e) => console.error(e))
  }

  const handleLogout = () => {
    instance.logoutPopup().catch((e) => console.error(e))
  }

  return (
    <div>
      {account ? (
        <>
          <p>Signed in as {account.name}</p>
          <button onClick={handleLogout}>Logout</button>
        </>
      ) : (
        <button onClick={handleLogin}>Sign In</button>
      )}
    </div>
  )
}
