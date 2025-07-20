import { useState, useEffect } from 'react'
import { useAuth } from '@/stores/authStore'
import { getPrompts } from '@/lib/api'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import { ProfileDropdown } from '@/components/profile-dropdown'
import { Search } from '@/components/search'
import { ThemeSwitch } from '@/components/theme-switch'
import { columns } from './components/columns'
import { DataTable } from './components/data-table'
import { PromptsDialogs } from './components/prompts-dialogs'
import { PromptPrimaryButtons } from './components/prompts-primary-buttons'
import PromptsProvider from './context/prompts-context'
import { PromptData } from './data/schema'

export default function Prompts() {
  const [data, setData] = useState<PromptData[] | null>(null)
  const [refreshFlag, setRefreshFlag] = useState(0)
  const auth = useAuth()
  const jwt_token = `Bearer ${auth.accessToken}`

  useEffect(() => {
    getPrompts(jwt_token)
      .then(setData)
      .catch(() => setData(null))
  }, [refreshFlag])
  console.log(data)
  return (
    <PromptsProvider>
      <Header fixed>
        <Search />
        <div className='ml-auto flex items-center space-x-4'>
          <ThemeSwitch />
          <ProfileDropdown />
        </div>
      </Header>

      <Main>
        <div className='mb-2 flex flex-wrap items-center justify-between space-y-2 gap-x-4'>
          <div>
            <h2 className='text-2xl font-bold tracking-tight'>Prompts</h2>
            <p className='text-muted-foreground'>
              Here&apos;s a list of your prompts!
            </p>
          </div>
          <PromptPrimaryButtons />
        </div>
        <div className='-mx-4 flex-1 overflow-auto px-4 py-1 lg:flex-row lg:space-y-0 lg:space-x-12'>
          <DataTable data={data || []} columns={columns} />
        </div>
      </Main>

      <PromptsDialogs onRefresh={() => setRefreshFlag((n) => n + 1)} />
    </PromptsProvider>
  )
}
