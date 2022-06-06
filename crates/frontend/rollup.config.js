import rust from "@wasm-tool/rollup-plugin-rust";
import copy from "@guanghechen/rollup-plugin-copy";
import scss from "rollup-plugin-scss";

export default {
  input: {
    app: "src/index.mjs",
  },
  output: {
    dir: "dist",
    format: "esm",
  },
  plugins: [
    rust(),
    scss({
      output: "dist/bundle.css",
      outputStyle: "compressed",
    }),
    copy({ targets: [{ src: "src/index.html", dest: "dist" }] }),
  ],
};
