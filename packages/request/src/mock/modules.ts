import type { MockWrapper } from '@alova/mock'

interface MockModule {
  default: MockWrapper
}

const modules: Record<string, MockModule> = import.meta.glob('../api/modules/**/*.mock.ts', { eager: true })

const mockHandlers = Object.values(modules).map(module => module.default)

export default mockHandlers
