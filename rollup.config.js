import { defineConfig } from "rollup";
import terser from "@rollup/plugin-terser";

export default defineConfig([
  {
    input: "copy.js",
    output: {
      file: "copy.min.js",
      format: "es",
    },
    treeshake: "smallest",
    plugins: [
      terser(),
    ],
  }
]);
