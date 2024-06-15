import axios from 'axios'
import type { AxiosInstance, InternalAxiosRequestConfig, AxiosResponse } from 'axios'

const instance: AxiosInstance = axios.create({
  baseURL: '',
  headers: {
    'Content-Type': 'application/json'
  },
  timeout: 30000
})

instance.interceptors.request.use(
  (req: InternalAxiosRequestConfig<any>) => {
    if (!req.headers!['token']) {
      req.headers!['token'] = `Bearn ${localStorage.getItem('token')!}`
    }
    return req
  },
  (err) => Promise.reject(err)
)

instance.interceptors.response.use(
  (res: AxiosResponse) => {
    // todo 响应处理
    // if (res.status === 400 || res.status === 403) {
    // }

    return res
  },
  (err) => Promise.reject(err)
)

export default instance
