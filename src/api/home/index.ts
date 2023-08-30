import { http } from '@/request'

export interface GetDataArgs {
  [k: string]: any
}

export function getData(params: GetDataArgs) {
  return http.Get('api/getData', { params })
}
