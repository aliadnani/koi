module.exports = {
  env: {
    browser: true,
    es2021: true
  },
  ignorePatterns: ["vite.config.ts", "src/vite-env.d.ts"],
  extends: [
    'plugin:react/recommended',
    'standard-with-typescript',
    'prettier'
  ],
  overrides: [
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    tsconfigRootDir: __dirname,
    project: ["./tsconfig.json"]
  },
  plugins: [
    'react'
  ],
  rules: {
    "react/react-in-jsx-scope": "off",
    "@typescript-eslint/explicit-function-return-type": "off",
  }
}
