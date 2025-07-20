import axios from 'axios'
import { toast } from 'sonner'
import { PromptData } from '@/features/prompts/data/schema'
import {
  CommitInfo,
  UserInfo,
  CreateUserParam,
  UpdateUserParam,
} from '@/features/type'
import { User, userListSchema } from '@/features/users/data/schema'

export async function signIn(
  email: string,
  password: string
): Promise<UserInfo> {
  return axios
    .post('/api/user/signin', { email, password })
    .then((res) => {
      toast.success('Login successful!', { duration: 2000 })
      return res.data.result
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Login failed', {
          duration: 2000,
        })
      } else {
        toast.error('Network error', { duration: 2000 })
      }
    })
}

export async function signUp(
  email: string,
  username: string,
  password: string
): Promise<UserInfo> {
  return axios
    .post('/api/user/signup', { email, username, password })
    .then((res) => {
      toast.success('Sign up successful!', { duration: 2000 })
      return res.data.result
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Login failed', {
          duration: 2000,
        })
      } else {
        toast.error('Network error', { duration: 2000 })
      }
    })
}

export async function listVersion(
  prompt_id: number,
  jwt_token: string
): Promise<string[]> {
  return axios
    .get('/api/prompt/list_version', {
      params: { prompt_id },
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      return (res.data.result as string[]).reverse()
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || `Request failed ${err}`, {
          duration: 2000,
        })
      } else {
        toast.error(`List version failed: ${err}`, { duration: 2000 })
      }
      throw err
    })
}
export async function listCommits(
  prompt_id: number,
  version: string,
  jwt_token: string
): Promise<string[]> {
  return axios
    .get('/api/prompt/list_commit', {
      params: { prompt_id, version },
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      return (res.data.result as string[]).reverse()
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Auth failed', {
          duration: 2000,
        })
      } else {
        toast.error(`List commit failed: ${err}`, { duration: 2000 })
      }
      throw err
    })
}

export async function createPrompt(
  name: string,
  jwt_token: string
): Promise<void> {
  return axios
    .post(
      '/api/prompt/create_prompt',
      { name },
      { headers: { Authorization: jwt_token } }
    )
    .then((res) => {
      toast.success(`Create prompt ${res.data.status}!`, { duration: 2000 })
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Auth failed', {
          duration: 2000,
        })
      } else {
        toast.error(`Create prompt failed: ${err}`, { duration: 2000 })
      }
      throw err
    })
}

export async function createVersion(
  prompt_id: number,
  version: string,
  jwt_token: string
): Promise<void> {
  return axios
    .post(
      '/api/prompt/create_node',
      {
        prompt_id,
        version,
      },
      { headers: { Authorization: jwt_token } }
    )
    .then((res) => {
      const status = res.data.status
      toast.success(`Create version ${status}!`, { duration: 2000 })
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Auth failed', {
          duration: 2000,
        })
      } else {
        toast.error(`Create version failed: ${err}`, { duration: 2000 })
      }
    })
}

export async function createCommit(
  commitInfo: CommitInfo,
  jwt_token: string
): Promise<void> {
  return axios
    .post('/api/prompt/create_commit', commitInfo, {
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      toast.success(`Create commit ${res.data.status}!`, { duration: 2000 })
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Auth failed', {
          duration: 2000,
        })
      } else {
        toast.error(`Create commit failed: ${err}`, { duration: 2000 })
      }
      throw err
    })
}

export async function getPrompts(jwt_token: string): Promise<PromptData[]> {
  return axios
    .get('/api/prompt/query', { headers: { Authorization: jwt_token } })
    .then((res) => {
      return (res.data.result as PromptData[]).sort(
        (a, b) =>
          new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
      )
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.msg || err, { duration: 2000 })
      } else {
        toast.error('Network error', { duration: 2000 })
      }
      throw err
    })
}

export async function getCommitContent(
  prompt_id: number,
  version: string,
  commit_id: string,
  jwt_token: string
): Promise<string> {
  return axios
    .get('/api/prompt/content', {
      params: { prompt_id, version, commit_id },
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      return res.data.result
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.msg || err, { duration: 2000 })
      } else {
        toast.error('Network error', { duration: 2000 })
      }
    })
}

export async function deletePrompt(
  id: number,
  jwt_token: string
): Promise<void> {
  return axios
    .delete('/api/prompt', {
      params: { id },
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      toast.success(`Delete prompt ${id} ${res.data.status}!`, {
        duration: 2000,
      })
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Auth failed', {
          duration: 2000,
        })
      } else {
        toast.error(`Delete prompt ${id} failed: ${err}`, { duration: 2000 })
      }
    })
}

export async function rollbackCommit(
  prompt_id: number,
  version: string,
  commit_id: string,
  jwt_token: string
): Promise<void> {
  return axios
    .post(
      '/api/prompt/rollback',
      { prompt_id, version, commit_id },
      {
        headers: { Authorization: jwt_token },
      }
    )
    .then((res) => {
      toast.success(`Rollback prompt  ${res.data.status}!`, { duration: 2000 })
    })
    .catch((err) => {
      if (axios.isAxiosError(err)) {
        toast.error(err.response?.data.message || 'Auth failed', {
          duration: 2000,
        })
      } else {
        toast.error(`Rollback prompt  failed: ${err}`, { duration: 2000 })
      }
    })
}

export async function listUsers(jwt_token: string): Promise<User[]> {
  return axios
    .get('/api/control/list/user', {
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      return userListSchema.parse(res.data.result)
    })
    .catch((err) => {
      toast.error(`Rollback prompt  failed: ${err}`, { duration: 2000 })
      throw err
    })
}

export async function deleteUser(
  user_id: number,
  jwt_token: string
): Promise<void> {
  return axios
    .delete(`/api/control/user/${user_id}`, {
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      toast.success(res.data.msg, { duration: 2000 })
    })
    .catch((err) => {
      toast.error(`User deleted failed: ${err}`, { duration: 2000 })
    })
}

export async function addUser(
  user_info: CreateUserParam,
  jwt_token: string
): Promise<void> {
  return axios
    .post('/api/control/add/user', user_info, {
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      toast.success(res.data.msg, { duration: 2000 })
    })
    .catch((err) => {
      toast.error(`User add failed: ${err}`, { duration: 2000 })
    })
}

export async function updateUser(
  user_id: number,
  user_info: UpdateUserParam,
  jwt_token: string
): Promise<void> {
  return axios
    .post(`/api/control/update/user/${user_id}`, user_info, {
      headers: { Authorization: jwt_token },
    })
    .then((res) => {
      toast.success(res.data.msg, { duration: 2000 })
    })
    .catch((err) => {
      toast.error(`User update failed: ${err}`, { duration: 2000 })
    })
}
