function getOAuthParams() {
    const params = new URLSearchParams(window.location.search)
    console.log("OAuth Params:", params.toString())
}

getOAuthParams()
