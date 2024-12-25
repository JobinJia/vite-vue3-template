import { cleanup } from '@testing-library/vue'
import { afterEach } from 'vitest'
import '@testing-library/jest-dom'

// 每个测试后自动清理
afterEach(() => {
  cleanup()
})
