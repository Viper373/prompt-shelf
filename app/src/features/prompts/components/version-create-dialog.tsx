import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { toast } from 'sonner'
import { useAuth } from '@/stores/authStore'
import { createVersion } from '@/lib/api'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { PromptData } from '../data/schema'
import { VersionListWithDialog } from './version-list-dialog'

const formSchema = z.object({
  version: z.string().min(1, 'Version is required'),
})

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: PromptData
}

export function VersionCreateDialog({ open, onOpenChange, currentRow }: Props) {
  const auth = useAuth()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: { version: 'v0.0.1' },
  })

  const onSubmit = async (values: { version: string }) => {
    if (!currentRow?.id) {
      toast.error('Prompt not selected')
      return
    }
    const token = `Bearer ${auth.accessToken}`
    await createVersion(currentRow?.id, values.version, token)
    onOpenChange(false)
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(val) => {
        onOpenChange(val)
        form.reset()
      }}
    >
      <DialogContent className='gap-2 sm:max-w-sm'>
        <DialogHeader className='text-left'>
          <DialogTitle>Create Version</DialogTitle>
          <DialogDescription>Please input a version</DialogDescription>
          <VersionListWithDialog
            promptId={currentRow?.id}
            authToken={`Bearer ${auth.accessToken}`}
          />
        </DialogHeader>
        <Form {...form}>
          <form id='version-create-form' onSubmit={form.handleSubmit(onSubmit)}>
            <FormField
              control={form.control}
              name='version'
              render={({ field }) => (
                <FormItem className='mb-2 space-y-1'>
                  <FormLabel>Version</FormLabel>
                  <FormControl>
                    <Input {...field} className='h-8' />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <DialogFooter className='gap-2'>
          <DialogClose asChild>
            <Button variant='outline'>Close</Button>
          </DialogClose>
          <Button type='submit' form='version-create-form'>
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
