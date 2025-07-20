export interface CommitInfo {
  prompt_id: number
  version: string
  desp: string
  as_latest: boolean
  content: string
}

export interface UserInfo {
  username: string
  email: string
  id: number
  role: string
  token: string
}

export interface CreateUserParam {
  username: string
  email: string
  role: string
  password: string
  valid: boolean
}

export interface UpdateUserParam {
  username?: string
  email?: string
  role?: string
  password?: string
  valid?: boolean
}
