import {
  IconLayoutDashboard,
  IconPalette,
  IconSettings,
  IconUsers,
} from '@tabler/icons-react'
import { useAuth } from '@/stores/authStore'
import { type SidebarData } from '../types'

export function useSidebarData(): SidebarData {
  const auth = useAuth()
  const isSuperAdmin = auth.user?.role === 'super_admin'
  return {
    user: auth.user || { username: '', email: '', role: '' },
    navGroups: [
      {
        title: 'General',
        items: [
          {
            title: 'Prompts',
            url: '/',
            icon: IconLayoutDashboard,
          },
          ...(isSuperAdmin
            ? [
                {
                  title: 'Users',
                  url: '/users' as const,
                  icon: IconUsers,
                },
              ]
            : []),
        ],
      },
      {
        title: 'Other',
        items: [
          {
            title: 'Settings',
            icon: IconSettings,
            items: [
              {
                title: 'Appearance',
                url: '/settings/appearance',
                icon: IconPalette,
              },
            ],
          },
          // {
          // 	title: 'Help Center',
          // 	url: '/help-center',
          // 	icon: IconHelp,
          // },
        ],
      },
    ],
  }
}
