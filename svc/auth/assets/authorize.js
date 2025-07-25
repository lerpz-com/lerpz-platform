const client = axios.create({
  baseURL: "/api/",
  timeout: 1000,
  allowAbsoluteURLs: false,
  params: getOAuthParams(),
});

function getOAuthParams() {
  const params = new URLSearchParams(window.location.search);
  return params;
}

function handleLogin(event) {
  event.preventDefault();
  const formData = new FormData(event.target);
  const res = client.post("/login", formData);
  console.log("Login response:", res);
}

document.addEventListener("DOMContentLoaded", () => {
  const form = document.querySelector("form");
  if (form) {
    form.addEventListener("submit", handleLogin);
  }
});
