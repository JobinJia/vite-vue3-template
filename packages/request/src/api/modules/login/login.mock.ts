import type { MockServerRequest } from '@alova/mock'
import type { LoginResponse } from './login'
import { defineMock } from '@alova/mock'

export default defineMock({
  // 登录接口
  '[POST]/auth/login': (_: { data: { phone: string, code: string } }): LoginResponse => {
    return {
      accessToken: 'mock-token',
      refreshToken: 'mock-refresh-token',
      expireIn: 7200,
      refreshExpireIn: null,
    }
  },
  // 发送验证码接口
  '[GET]/auth/code': (params: MockServerRequest) => {
    const { phone } = params.query
    // 模拟发送验证码
    if (/^1[3-9]\d{9}$/.test(phone)) {
      return {
        code: Math.random.toString().substring(2, 8),
        data: null,
        message: '验证码发送成功',
      }
    }
    return {
      status: 400,
      data: null,
      message: '手机号格式错误',
    }
  },
})
