import { defineConfig } from "vite";
import path from "node:path";
import fs from "node:fs/promises";
import ts from "typescript";

const root = path.resolve("/workspace/a2ui-test");
const vendorRoot = path.resolve(root, "vendor/a2ui");
const tsCompilerOptions = {
  experimentalDecorators: true,
  useDefineForClassFields: false,
};

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        index: path.resolve(root, "index.html"),
      },
    },
  },
  esbuild: {
    tsconfigRaw: {
      compilerOptions: tsCompilerOptions,
    },
  },
  optimizeDeps: {
    noDiscovery: true,
    exclude: ["@a2ui/web_core", "@a2ui/spec", "@a2ui/lit"],
  },
  resolve: {
    alias: [
      {
        find: /^@a2ui\/web_core$/,
        replacement: path.resolve(
          root,
          "vendor/a2ui/renderers/web_core/src/v0_8/index.ts",
        ),
      },
      {
        find: /^@a2ui\/web_core\/(.+)$/,
        replacement: path.resolve(
          root,
          "vendor/a2ui/renderers/web_core/src/v0_8/$1.ts",
        ),
      },
      {
        find: /^@a2ui\/lit$/,
        replacement: path.resolve(root, "vendor/a2ui/renderers/lit/src/index.ts"),
      },
      {
        find: /^@a2ui\/lit\/(.+)$/,
        replacement: path.resolve(root, "vendor/a2ui/renderers/lit/src/$1.ts"),
      },
      {
        find: /^@a2ui\/spec$/,
        replacement: path.resolve(root, "vendor/a2ui/specification/v0_8/json"),
      },
      {
        find: /^@a2ui\/spec\/(.+)$/,
        replacement: path.resolve(root, "vendor/a2ui/specification/v0_8/json/$1"),
      },
    ],
  },
  plugins: [
    {
      name: "transpile-vendored-a2ui-ts",
      enforce: "pre",
      async load(id) {
        if (!id.startsWith(vendorRoot) || !id.endsWith(".ts")) {
          return null;
        }

        const source = await fs.readFile(id, "utf8");
        const result = ts.transpileModule(source, {
          compilerOptions: {
            module: ts.ModuleKind.ESNext,
            target: ts.ScriptTarget.ES2022,
            experimentalDecorators: true,
            useDefineForClassFields: false,
            sourceMap: true,
            inlineSources: true,
          },
          fileName: id,
        });

        return {
          code: result.outputText,
          map: result.sourceMapText ?? null,
        };
      },
    },
  ],
});
