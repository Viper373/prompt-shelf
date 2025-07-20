import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useAuth } from '@/stores/authStore'
import { createPrompt } from '@/lib/api'
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

const formSchema = z.object({
  name: z.string().min(1, 'Prompt name is required'),
})

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function PromptCreateDialog({ open, onOpenChange }: Props) {
  const auth = useAuth()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: { name: '' },
  })

  const onSubmit = async (values: { name: string }) => {
    const token = `Bearer ${auth.accessToken}`
    createPrompt(values.name, token).finally(() => onOpenChange(false))
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
          <DialogTitle>Create Prompt</DialogTitle>
          <DialogDescription>Please input a prompt name</DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form id='prompt-create-form' onSubmit={form.handleSubmit(onSubmit)}>
            <FormField
              control={form.control}
              name='name'
              render={({ field }) => (
                <FormItem className='mb-2 space-y-1'>
                  <FormLabel>Name</FormLabel>
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
          <Button type='submit' form='prompt-create-form'>
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
