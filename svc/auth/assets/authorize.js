const instance = axios.create({
  baseURL: '/api',
  timeout: 1000,
  headers: {'X-Custom-Header': 'foobar'}
});

function getOAuthParams() {
    const params = new URLSearchParams(window.location.search)
    console.log("OAuth Params:", params.toString())
}

getOAuthParams()
