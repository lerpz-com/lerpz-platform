import { defineConfig } from "@solidjs/start/config";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
	ssr: true,
 server: {
    preset: "deno-server",
  },
	vite: {
		plugins: [tailwindcss()],
	},
});
