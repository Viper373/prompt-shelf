import { z } from 'zod'

const userSchema = z.object({
  id: z.number(),
  username: z.string(),
  email: z.string(),
  valid: z.boolean(),
  role: z.string(),
  created_at: z.coerce.date(),
  updated_at: z.coerce.date(),
})
export type User = z.infer<typeof userSchema>

export const userListSchema = z.array(userSchema)
