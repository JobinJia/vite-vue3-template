import antfu from '@antfu/eslint-config'

export default antfu({
  unocss: true,
  formatters: true,
  rules: {
    'no-console': 'off',
    'curly': ['error', 'all'],
    'style/brace-style': ['error', '1tbs', { allowSingleLine: false }],
  },
  stylistic: {
    'curly': ['error', 'all'],
    'style/brace-style': ['error', '1tbs', { allowSingleLine: false }],
  },
})
