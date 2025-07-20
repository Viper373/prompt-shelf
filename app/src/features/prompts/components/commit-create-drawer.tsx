import { useEffect, useState } from 'react'
import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { toast } from 'sonner'
import { useAuth } from '@/stores/authStore'
import { createCommit, listVersion } from '@/lib/api'
import { Button } from '@/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
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
import { CommitInfo } from '@/features/type'
import { PromptData } from '../data/schema'

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: PromptData
}

const formSchema = z.object({
  version: z.string().min(1, 'Version is required.'),
  desp: z.string(),
  as_latest: z.boolean(),
  content: z.string().min(1, 'Please input prompt content.'),
})
type CommitForm = z.infer<typeof formSchema>

export function CommitCreateDrawer({ open, onOpenChange, currentRow }: Props) {
  const auth = useAuth()
  const [disableSubmit, setDisableSubmit] = useState(false)
  const [versions, setVersions] = useState<string[]>([])

  useEffect(() => {
    if (open && currentRow?.id) {
      const fetchVersions = async () => {
        try {
          const res = await listVersion(
            currentRow?.id,
            `Bearer ${auth.accessToken}`
          )
          if (res.length === 0) {
            toast.error('No versions available.', { duration: 2000 })
            setDisableSubmit(true)
          } else {
            setDisableSubmit(false)
            setVersions(res)
          }
        } catch (error) {
          console.error('Failed to fetch versions', error)
        }
      }
      fetchVersions()
    }
  }, [open, currentRow])

  const form = useForm<CommitForm>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      version: currentRow?.latest_version || 'v0.0.1',
      desp: '',
      as_latest: true,
      content: '',
    },
  })

  const onSubmit = async (data: CommitForm) => {
    if (!currentRow?.id) {
      toast.error('Prompt not selected')
      return
    }
    const token = `Bearer ${auth.accessToken}`
    const commitInfo: CommitInfo = {
      prompt_id: currentRow?.id,
      ...data,
    }
    await createCommit(commitInfo, token)
    onOpenChange(false)
    form.reset()
    // showSubmittedData(data)
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
          <SheetTitle> Commit</SheetTitle>
          <SheetDescription>Add a new commit</SheetDescription>
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
              name='desp'
              render={({ field }) => (
                <FormItem className='space-y-1'>
                  <FormLabel>Desp</FormLabel>
                  <FormControl>
                    <Input {...field} placeholder='Enter a commit desp' />
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
                      placeholder='Enter a content'
                      className='max-h-[300px] min-h-[200px] resize-y overflow-auto p-4 text-base'
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
            Commit
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
