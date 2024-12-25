import { defineConfig } from 'cypress'

export default defineConfig({
  e2e: {
    baseUrl: 'http://localhost:5173',
    supportFile: 'src/cypress/support/e2e.ts',
    specPattern: 'src/cypress/e2e/**/*.cy.{js,jsx,ts,tsx}',
    fixturesFolder: 'src/cypress/fixtures',
  },
  component: {
    devServer: {
      framework: 'vue',
      bundler: 'vite',
    },
    supportFile: 'src/cypress/support/component.ts',
    specPattern: '**/*.cy.{js,jsx,ts,tsx}',
    fixturesFolder: 'src/cypress/fixtures',
  },
})
