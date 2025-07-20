import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import { Search } from '@/components/search'
import { ThemeSwitch } from '@/components/theme-switch'
import SettingsAppearance from './appearance'

export default function Settings() {
  return (
    <>
      {/* ===== Top Heading ===== */}
      <Header>
        <Search />
        <div className='ml-auto flex items-center space-x-4'>
          <ThemeSwitch />
        </div>
      </Header>

      <Main fixed>
        <SettingsAppearance />
      </Main>
    </>
  )
}

// const sidebarNavItems = [
// 	{
// 		title: 'Appearance',
// 		icon: <IconPalette size={18} />,
// 		href: '/settings/appearance',
// 	},
// ]
