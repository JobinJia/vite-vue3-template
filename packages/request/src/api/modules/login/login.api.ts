import type { BaseResponse } from '../../../types'
import type { LoginRequest, LoginResponse } from './login'
import { http } from '../../../fetch'

export const LoginApi = {
  login: (data: LoginRequest) =>
    http.Post<BaseResponse<LoginResponse>>('/auth/smsLogin', data),
  // Others....
}
