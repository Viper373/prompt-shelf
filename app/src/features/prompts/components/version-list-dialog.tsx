import { useEffect, useState } from 'react'
import { toast } from 'sonner'
import { listVersion } from '@/lib/api'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogTitle } from '@/components/ui/dialog'

interface Props {
  promptId?: number
  authToken: string
}

export function VersionListWithDialog({ promptId, authToken }: Props) {
  if (!promptId) return null
  const [versions, setVersions] = useState<string[]>([])
  const [showAll, setShowAll] = useState(false)
  const maxShow = 3

  useEffect(() => {
    if (!promptId) return
    listVersion(promptId, authToken)
      .then((res) => {
        setVersions(res)
      })
      .catch(() => {
        toast.error('Failed to load versions')
      })
  }, [promptId, authToken])

  return (
    <>
      <div className='mt-2 text-sm text-gray-600'>
        <div className='mb-1 font-semibold'>
          Existing versions (已存在版本):
        </div>
        {versions.slice(0, maxShow).map((v) => (
          <div key={v} className='flex items-center gap-2'>
            <span className='inline-block h-2 w-2 rounded-full bg-gray-400' />
            <span>{v}</span>
          </div>
        ))}
        {versions.length > maxShow && (
          <button
            className='mt-1 text-xs text-blue-600 hover:underline'
            onClick={() => setShowAll(!showAll)}
            type='button'
          >
            {showAll
              ? 'Hide versions'
              : `Show all versions (${versions.length})`}
          </button>
        )}
      </div>

      <Dialog open={showAll} onOpenChange={setShowAll}>
        <DialogContent className='max-w-md p-6'>
          <DialogTitle className='mb-4 text-lg font-semibold'>
            All versions
          </DialogTitle>
          <div className='max-h-60 overflow-auto rounded-md border border-gray-200 px-3 py-2 shadow-inner'>
            {versions.map((v) => (
              <div
                key={v}
                className='border-b border-gray-100 py-2 text-sm text-gray-700 last:border-none'
              >
                {v}
              </div>
            ))}
          </div>
          <div className='mt-6 flex justify-center'>
            <Button onClick={() => setShowAll(false)} className='w-24'>
              Close
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  )
}
