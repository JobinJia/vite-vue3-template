import type { BaseResponse } from 'src/types'
import type { LoginRequest, LoginResponse } from './login.d'
import { http } from 'src/fetch'

export const loginApi = {
  login: (data: LoginRequest) =>
    http.Post<BaseResponse<LoginResponse>>('/auth/smsLogin', data),
  // Others....
}
