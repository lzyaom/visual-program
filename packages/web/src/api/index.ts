import axios from './instance'

export const getProgram = (params: any) => axios.get('/api/program', { params })
export const createProgram = (data: any) => axios.post('/api/program', data)
export const deleteProgram = (id: string) => axios.delete(`/api/program/${id}`)
export const updateProgram = (id: string, data: any) => axios.put(`/api/program/${id}`, data)
