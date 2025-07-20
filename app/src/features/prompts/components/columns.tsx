import { ColumnDef } from '@tanstack/react-table'
import { Checkbox } from '@/components/ui/checkbox'
import { PromptData } from '../data/schema'
import { DataTableColumnHeader } from './data-table-column-header'
import { DataTableRowActions } from './data-table-row-actions'

export const columns: ColumnDef<PromptData>[] = [
  {
    id: 'select',
    header: ({ table }) => (
      <Checkbox
        checked={
          table.getIsAllPageRowsSelected() ||
          (table.getIsSomePageRowsSelected() && 'indeterminate')
        }
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        aria-label='Select all'
        className='translate-y-[2px]'
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
        aria-label='Select row'
        className='translate-y-[2px]'
      />
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'id',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='ID' />
    ),
    cell: ({ row }) => <div className='w-[80px]'>{row.getValue('id')}</div>,
  },
  {
    accessorKey: 'prompt.name',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Prompt Name' />
    ),
    cell: ({ row }) => row.original.prompt.name,
  },
  {
    accessorKey: 'latest_version',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Latest Version' />
    ),
    cell: ({ row }) => row.getValue('latest_version'),
  },
  {
    accessorKey: 'latest_commit',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Latest Commit' />
    ),
    cell: ({ row }) => row.getValue('latest_commit'),
  },
  {
    accessorKey: 'created_at',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Created' />
    ),
    cell: ({ row }) =>
      new Date(row.getValue('created_at')).toLocaleString('zh-CN'),
  },
  {
    accessorKey: 'updated_at',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Updated' />
    ),
    cell: ({ row }) =>
      new Date(row.getValue('updated_at')).toLocaleString('zh-CN'),
  },
  {
    id: 'actions',
    cell: ({ row }) => <DataTableRowActions row={row} />,
  },
]
