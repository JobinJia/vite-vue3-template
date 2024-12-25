import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vitest/config'

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'happy-dom',
    include: [
      // 测试文件匹配模式
      'src/**/*.{test,spec}.{js,ts,jsx,tsx}',
      // 组件测试
      'src/components/**/__tests__/*.{js,ts,jsx,tsx}',
    ],
    exclude: [
      '**/node_modules/**',
      '**/dist/**',
      '**/cypress/**',
      '**/.{idea,git,cache,output,temp}/**',
    ],
  },
})
