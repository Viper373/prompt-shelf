import React, { useState } from 'react'
import useDialogState from '@/hooks/use-dialog-state'
import { PromptData } from '../data/schema'

type PromptsDialogType = 'create' | 'version' | 'commit' | 'delete' | 'rollback'

interface PromptsContextType {
  open: PromptsDialogType | null
  setOpen: (str: PromptsDialogType | null) => void
  currentRow: PromptData | null
  setCurrentRow: React.Dispatch<React.SetStateAction<PromptData | null>>
}

const PromptsContext = React.createContext<PromptsContextType | null>(null)

interface Props {
  children: React.ReactNode
}

export default function PromptsProvider({ children }: Props) {
  const [open, setOpen] = useDialogState<PromptsDialogType>(null)
  const [currentRow, setCurrentRow] = useState<PromptData | null>(null)
  return (
    <PromptsContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </PromptsContext>
  )
}

// eslint-disable-next-line react-refresh/only-export-components
export const usePrompts = () => {
  const promptsContext = React.useContext(PromptsContext)

  if (!promptsContext) {
    throw new Error('usePrompts has to be used within <PromptsContext>')
  }

  return promptsContext
}
