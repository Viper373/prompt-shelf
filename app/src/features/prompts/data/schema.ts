import { z } from 'zod'

const CommitSchema = z.object({
  author: z.string().email(),
  commit_id: z.string().uuid(),
  created_at: z.string().datetime(),
  desp: z.string(),
})

const NodeSchema = z.object({
  commits: z.array(CommitSchema),
  updated_at: z.string().datetime(),
  version: z.string(),
})

const PromptSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  nodes: z.array(NodeSchema),
})

export const DataSchema = z.object({
  created_at: z.string().datetime(),
  id: z.number(),
  latest_commit: z.string().uuid().nullable(),
  latest_version: z.string().nullable(),
  org_id: z.nullable(z.any()),
  prompt: PromptSchema,
  updated_at: z.string().datetime(),
  user_id: z.number(),
})

type PromptElement = z.infer<typeof PromptSchema>
export type PromptData = z.infer<typeof DataSchema>
