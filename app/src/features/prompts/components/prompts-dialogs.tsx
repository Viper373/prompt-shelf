import { toast } from 'sonner'
import { useAuth } from '@/stores/authStore'
import { deletePrompt } from '@/lib/api'
import { showSubmittedData } from '@/utils/show-submitted-data'
import { ConfirmDialog } from '@/components/confirm-dialog'
import { usePrompts } from '@/features/prompts/context/prompts-context'
import { CommitCreateDrawer } from './commit-create-drawer'
import { PromptCreateDialog } from './prompt-create-dialog'
import { CommitRollbackDrawer } from './rollback-commit-drawer'
import { VersionCreateDialog } from './version-create-dialog'

interface PromptsDialogsProps {
  onRefresh: () => void
}
export function PromptsDialogs({ onRefresh }: PromptsDialogsProps) {
  const auth = useAuth()
  const { open, setOpen, currentRow, setCurrentRow } = usePrompts()
  const handleConform = async () => {
    if (!currentRow?.id) {
      toast.error('Prompt not selected', { duration: 2000 })
      return
    }
    const token = `Bearer ${auth.accessToken}`
    await deletePrompt(currentRow.id, token)
    onRefresh()
    setOpen(null)
    setTimeout(() => {
      setCurrentRow(null)
    }, 500)
  }
  return (
    <>
      <PromptCreateDialog
        key='prompt-create'
        open={open === 'create'}
        onOpenChange={(v) => {
          if (!v) onRefresh()
          setOpen('create')
        }}
      />

      {currentRow && (
        <>
          <VersionCreateDialog
            key={`version-create-${currentRow.id}`}
            open={open === 'version'}
            onOpenChange={(v) => {
              if (!v) onRefresh()
              setOpen('version')
            }}
            currentRow={currentRow}
          />
          <CommitCreateDrawer
            key={`prompt-commit-${currentRow.id}`}
            open={open === 'commit'}
            onOpenChange={(v) => {
              if (!v) onRefresh()
              setOpen('commit')
              setTimeout(() => {
                setCurrentRow(null)
              }, 500)
            }}
            currentRow={currentRow}
          />
          <CommitRollbackDrawer
            key={`prompt-commit-${currentRow.id}`}
            open={open === 'rollback'}
            onOpenChange={(v) => {
              if (!v) onRefresh()
              setOpen('rollback')
              setTimeout(() => {
                setCurrentRow(null)
              }, 500)
            }}
            currentRow={currentRow}
          />

          <ConfirmDialog
            key='prompt-delete'
            destructive
            open={open === 'delete'}
            onOpenChange={() => {
              setOpen('delete')
              setTimeout(() => {
                setCurrentRow(null)
              }, 500)
            }}
            handleConfirm={handleConform}
            className='max-w-md'
            title={`Delete this prompt: ${currentRow.id} ?`}
            desc={
              <>
                You are about to delete a prompt with the ID{' '}
                <strong>{currentRow.id}</strong>. <br />
                This action cannot be undone.
              </>
            }
            confirmText='Delete'
          />
        </>
      )}
    </>
  )
}
