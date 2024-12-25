import type { MockServerRequest } from '@alova/mock'
import type { TestResponse } from './test'
import { defineMock } from '@alova/mock'

export default defineMock({
  // Example Mock Api TODO
  // eslint-disable-next-line unused-imports/no-unused-vars
  '[POST]/test/example': (params: MockServerRequest): TestResponse => {
    return {

    } as TestResponse
  },
})
