import Cookies from 'js-cookie'
import { create } from 'zustand'
import { AuthUser } from '@/components/layout/types'

const ACCESS_TOKEN = 'jwt_token'
const USER_INFO = 'user_info'
const initToken = Cookies.get(ACCESS_TOKEN) ?? ''

const initUser = Cookies.get(USER_INFO)

interface AuthState {
  auth: {
    user: AuthUser | null
    setUser: (user: AuthUser | null) => void
    accessToken: string
    setAccessToken: (accessToken: string) => void
    resetAccessToken: () => void
    reset: () => void
  }
}

export const useAuthStore = create<AuthState>()((set) => {
  return {
    auth: {
      user: initUser ? JSON.parse(initUser) : null,
      setUser: (user) =>
        set((state) => {
          Cookies.set(USER_INFO, JSON.stringify(user))
          return { ...state, auth: { ...state.auth, user } }
        }),
      accessToken: initToken,
      setAccessToken: (accessToken) =>
        set((state) => {
          Cookies.set(ACCESS_TOKEN, accessToken)
          return { ...state, auth: { ...state.auth, accessToken } }
        }),
      resetAccessToken: () =>
        set((state) => {
          Cookies.remove(ACCESS_TOKEN)
          return { ...state, auth: { ...state.auth, accessToken: '' } }
        }),
      reset: () =>
        set((state) => {
          Cookies.remove(ACCESS_TOKEN)
          Cookies.remove(USER_INFO)
          return {
            ...state,
            auth: { ...state.auth, user: null, accessToken: '' },
          }
        }),
    },
  }
})

export const useAuth = () => useAuthStore((state) => state.auth)
