import { useEffect, useState } from 'react'
import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { toast } from 'sonner'
import { useAuth } from '@/stores/authStore'
import {
  getCommitContent,
  listCommits,
  listVersion,
  rollbackCommit,
} from '@/lib/api'
import { Button } from '@/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from '@/components/ui/sheet'
import { Textarea } from '@/components/ui/textarea'
import { PromptData } from '../data/schema'

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: PromptData
}

const formSchema = z.object({
  version: z.string().min(1, 'Version is required.'),
  commit: z.string(),
  content: z.string().min(1, 'Please input prompt content.'),
})
type CommitForm = z.infer<typeof formSchema>

export function CommitRollbackDrawer({
  open,
  onOpenChange,
  currentRow,
}: Props) {
  const auth = useAuth()
  const [disableSubmit, setDisableSubmit] = useState(false)
  const [versions, setVersions] = useState<string[]>([])
  const [commits, setCommits] = useState<string[]>([])

  const form = useForm<CommitForm>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      version: currentRow?.latest_version || '',
      commit: currentRow?.latest_commit || '',
      content: '',
    },
  })

  useEffect(() => {
    if (open && currentRow?.id) {
      const fetchVersions = async () => {
        try {
          const res = await listVersion(
            currentRow?.id,
            `Bearer ${auth.accessToken}`
          )
          setVersions(res)
        } catch (error) {
          console.error('Failed to fetch versions', error)
        }
      }
      fetchVersions()
    }
  }, [open, currentRow])

  useEffect(() => {
    const version = form.getValues('version')
    if (version && currentRow?.id) {
      listCommits(currentRow.id, version, `Bearer ${auth.accessToken}`)
        .then((res) => {
          setCommits(res)
        })
        .catch(() => setCommits([]))
    }
  }, [form.watch('version')])

  useEffect(() => {
    const version = form.getValues('version')
    const commit = form.getValues('commit')
    if (version && commit && currentRow?.id && version) {
      getCommitContent(
        currentRow.id,
        version,
        commit,
        `Bearer ${auth.accessToken}`
      )
        .then((res) => form.setValue('content', res || ''))
        .catch(() => form.setValue('content', ''))
      setDisableSubmit(false)
    } else {
      setDisableSubmit(true)
    }
  }, [form.watch('version'), form.watch('commit')])

  const onSubmit = async (data: CommitForm) => {
    if (!currentRow?.id) {
      toast.error('Prompt not selected')
      return
    }
    const token = `Bearer ${auth.accessToken}`

    await rollbackCommit(currentRow.id, data.version, data.commit, token)
    onOpenChange(false)
    form.reset()
  }

  return (
    <Sheet
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
    >
      <SheetContent className='flex flex-col'>
        <SheetHeader className='text-left'>
          <SheetTitle> Rollback </SheetTitle>
          <SheetDescription>Rollback to a commit</SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id='commit-form'
            onSubmit={form.handleSubmit(onSubmit)}
            className='flex-1 space-y-5 px-4'
          >
            <FormField
              control={form.control}
              name='version'
              render={({ field }) => (
                <FormItem className='space-y-1'>
                  <FormLabel>Version</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className='w-full rounded border border-gray-300 px-3 py-2'
                    >
                      {versions.map((v) => (
                        <option key={v} value={v}>
                          {v}
                        </option>
                      ))}
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name='commit'
              render={({ field }) => (
                <FormItem className='space-y-1'>
                  <FormLabel>Commit</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className='w-full rounded border border-gray-300 px-3 py-2'
                    >
                      {commits.map((v) => (
                        <option key={v} value={v}>
                          {v}
                        </option>
                      ))}
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name='content'
              render={({ field }) => (
                <FormItem className='space-y-1'>
                  <FormLabel>Content</FormLabel>
                  <FormControl>
                    <Textarea
                      {...field}
                      readOnly
                      className='max-h-[300px] min-h-[200px] resize-y overflow-auto bg-gray-50 p-4 text-base'
                      placeholder='Selected commit content will show here'
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <SheetFooter className='gap-2'>
          <SheetClose asChild>
            <Button variant='outline'>Cancel</Button>
          </SheetClose>
          <Button form='commit-form' type='submit' disabled={disableSubmit}>
            Rollback
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
