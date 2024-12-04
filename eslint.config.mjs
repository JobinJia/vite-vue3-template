import antfu from '@antfu/eslint-config'

export default antfu({
  unocss: true,
  formatters: true,
  rules: {
    'no-console': 'off',
    'curly': 'off',
    'style/brace-style': ['error', '1tbs', { allowSingleLine: true }],
  },
  stylistic: {
    'style/brace-style': ['error', '1tbs', { allowSingleLine: true }],
  },
})
