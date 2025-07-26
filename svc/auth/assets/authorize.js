const client = axios.create({
  baseURL: "/",
  timeout: 10000,
  allowAbsoluteURLs: false,
});

function getOAuthParams() {
  return new URLSearchParams(window.location.search);
}

async function handleLogin(event) {
  event.preventDefault();
  
  try {
    const formData = new FormData(event.target);
    for (const [key, value] of getOAuthParams().entries()) {
      formData.append(key, value);
    }
    
    const response = await client.post("/login", formData);
    
    if (response.status === 200) {
      if (response.data.redirectUrl) {
        window.location.href = response.data.redirectUrl;
      } else {
        console.log("Login successful:", response.data);
      }
    }
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
