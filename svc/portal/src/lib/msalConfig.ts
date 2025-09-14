import { Configuration } from "@azure/msal-browser"

export const msalConfig: Configuration = {
  auth: {
    clientId: process.env.NEXT_PUBLIC_AZURE_CLIENT_ID || "",
    authority: `https://login.microsoftonline.com/${process.env.NEXT_PUBLIC_AZURE_TENANT_ID}`,
    redirectUri:
      process.env.NEXT_PUBLIC_REDIRECT_URI || "http://localhost:3000/dashboard"
  },
  cache: {
    cacheLocation: "sessionStorage",
    storeAuthStateInCookie: false
  }
}

export const loginRequest = {
  scopes: ["User.Read"]
}
