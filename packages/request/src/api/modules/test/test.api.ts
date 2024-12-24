import type { BaseResponse } from '../../../types'
import type { TestRequest, TestResponse } from './test'
import { http } from '../../../fetch'

export const TestApi = {
  // TODO: 添加API方法
  example: (data: TestRequest) =>
    http.Post<BaseResponse<TestResponse>>('/test/example', data),
}
