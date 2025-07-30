const client = axios.create({
  baseURL: "/",
  timeout: 10000,
  allowAbsoluteURLs: false,
});

function getParams() {
  return new URLSearchParams(window.location.search);
}

async function handleLogin(event) {
  event.preventDefault();

  try {
    const formData = new FormData(event.target);
    const response = await client.post("/login", formData);

    if (response.status !== 200) {
      alert("Something went wrong");
    }

    const loginUrl = "https://localhost:3001/oauth/2.0/authorize?";
    for (const [key, value] of getParams().entries()) {
      loginUrl.append(`${key}=${value}&`);
    }

    loginUrl.slice(0, -1);

    window.location.href = loginUrl;
  } catch (error) {
    console.error("Login failed:", error);

    if (error.response) {
      const errorMessage = error.response.data?.title || "Login failed";
      alert(errorMessage);
    } else if (error.request) {
      alert("Network error. Please try again.");
    } else {
      alert("An unexpected error occurred.");
    }
  }
}

document.addEventListener("DOMContentLoaded", () => {
  const form = document.querySelector("form");
  if (form) {
    form.addEventListener("submit", handleLogin);
  }
});
