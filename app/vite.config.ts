import { sveltekit } from "@sveltejs/kit/vite";
import wasmPack from "vite-plugin-wasm-pack";
import { defineConfig } from "vitest/config";

export default defineConfig({
	plugins: [wasmPack("../rust/"), sveltekit()],
	assetsInclude: ["**/*.txt*"],
	test: {
		include: ["src/**/*.{test,spec}.{js,ts}"],
	},
});
