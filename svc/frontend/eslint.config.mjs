import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

const eslintConfig = [
  ...compat.config({
    extends: [
      "next",
      "plugin:react/recommended",
      "plugin:import/recommended",
      "plugin:@typescript-eslint/recommended",
      "plugin:prettier/recommended",
    ],
    plugins: ["react", "react-hooks", "import", "@typescript-eslint"],
    rules: {
      "react/react-in-jsx-scope": "off",
      "comma-dangle": ["error", "never"],
    },
    parser: "@typescript-eslint/parser",
    parserOptions: {
      ecmaFeatures: {
        jsx: true,
      },
    },
    settings: {
      react: {
        version: "detect",
      },
    },
    ignorePatterns: ["node_modules/", ".next/", ".turbo/"],
  }),
];

export default eslintConfig;
