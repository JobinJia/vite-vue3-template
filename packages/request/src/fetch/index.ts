import { createAlova } from 'alova'
import adapterFetch from 'alova/fetch'
import vueHook from 'alova/vue'
import { mockAdapter } from 'src/mock'

// 创建请求实例
export const http = createAlova({
  baseURL: '',
  // baseURL: 'https://vphoto.aianywhere.cn/prod-api',
  statesHook: vueHook,
  requestAdapter: import.meta.env.DEV ? mockAdapter : adapterFetch(),
  responded: async (response: Response) => {
    const responseData = await response.json()
    if (import.meta.env.DEV) {
      const { body } = responseData
      return body
    } else {
      const { code, data, msg } = responseData
      if (responseData.code !== 200) {
        throw new Error(`${code}: ${msg}`)
      }
      return data
    }
  },
})
