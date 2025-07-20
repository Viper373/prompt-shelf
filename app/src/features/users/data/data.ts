import { IconShield, IconUsersGroup, IconUserShield } from '@tabler/icons-react'

export const userTypes = [
  {
    label: 'Superadmin',
    value: 'super_admin',
    icon: IconShield,
  },
  {
    label: 'Admin',
    value: 'admin',
    icon: IconUserShield,
  },
  {
    label: 'User',
    value: 'user',
    icon: IconUsersGroup,
  },
] as const
