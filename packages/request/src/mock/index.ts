import { createAlovaMockAdapter } from '@alova/mock'
import adapterFetch from 'alova/fetch'
import mockHandlers from './modules'

export const mockAdapter = createAlovaMockAdapter(mockHandlers, {
  // 全局控制是否启用mock接口，默认为true
  enable: true,

  // 非模拟请求适配器，用于未匹配mock接口时发送请求
  httpAdapter: adapterFetch(),

  // mock接口响应延迟，单位毫秒
  delay: 1000,

  // 是否打印mock接口请求信息
  mockRequestLogger: true,

  // 模拟接口回调，data为返回的模拟数据，你可以用它构造任何你想要的对象返回给alova
  // 以下为默认的回调函数，它适用于使用 `alova/fetch` 请求适配器
  // 如果你使用的是其他请求适配器，在模拟接口回调中请自定义返回适合适配器的数据结构
  onMockResponse: (data) => {
    const response = new Response(JSON.stringify(data))
    return {
      response,
      headers: response.headers,
    }
  },
})
