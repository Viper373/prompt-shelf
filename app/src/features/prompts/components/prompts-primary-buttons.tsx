import { IconPlus } from '@tabler/icons-react'
import { Button } from '@/components/ui/button'
import { usePrompts } from '@/features/prompts/context/prompts-context'

export function PromptPrimaryButtons() {
  const { setOpen } = usePrompts()
  return (
    <div className='flex gap-2'>
      <Button className='space-x-1' onClick={() => setOpen('create')}>
        <span>Create Prompt</span> <IconPlus size={18} />
      </Button>
    </div>
  )
}
