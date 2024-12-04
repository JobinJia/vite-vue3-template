export interface LoginRequest {
  account: string
  password: string
  validateCode: string
}

export interface LoginResponse {
  accessToken: string
  refreshToken: string
  expireIn: number
  refreshExpireIn: number | null
}

export interface SendCodeData {
  code: string
  data: null
  message: string
}
