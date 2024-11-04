import { createAlova } from 'alova'
import adapterFetch from 'alova/fetch'
import vueHook from 'alova/vue'

export const http = createAlova({
  baseURL: 'http://example.com',
  statesHook: vueHook,
  requestAdapter: adapterFetch(),
  // beforeRequest: randomDelayInterceptor,
  responded: (response: Response) => {
    if (response.status !== 200) {
      throw new Error(`[${response.status}]${response.statusText}`)
    }
    return response.json()
  },
})
