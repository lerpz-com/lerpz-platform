import { dirname } from "path"
import { fileURLToPath } from "url"
import { FlatCompat } from "@eslint/eslintrc"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

const compat = new FlatCompat({
  baseDirectory: __dirname
})

const eslintConfig = [
  ...compat.config({
    extends: [
      "plugin:@next/next/recommended",
      "plugin:@typescript-eslint/recommended",
      "plugin:import/recommended",
      "plugin:react/recommended",
      "plugin:prettier/recommended"
    ],
    plugins: [
      "@tanstack/query",
      "@typescript-eslint",
      "import",
      "react",
      "react-hooks"
    ],
    rules: {
      "react/react-in-jsx-scope": "off"
    },
    parser: "@typescript-eslint/parser",
    parserOptions: {
      ecmaFeatures: {
        jsx: true
      }
    },
    settings: {
      react: {
        version: "detect"
      },
      "import/resolver": {
        typescript: {
          project: "./tsconfig.json"
        }
      }
    },
    ignorePatterns: ["node_modules/", ".next/", ".turbo/"]
  })
]

export default eslintConfig
