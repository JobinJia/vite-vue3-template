import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { ref } from 'vue'

interface TreeNode {
  label: string
  value: string
  children?: TreeNode[]
}

export interface UserSelect {
  source: string
  sourcePath: string
  target: string
  targetPath: string
}

export function useAccountTree() {
  const treeData = ref<TreeNode[]>([])
  const message = useMessage()
  const buildTree = async (path: string): Promise<any[]> => {
    const originData: any[] = await invoke('list_directory_contents', { path })
    console.log(originData)
    return originData
  }

  const loadTree = async (basePath: string) => {
    treeData.value = await buildTree(basePath)
  }

  async function handleGoGO(basePath: string, userSelect: UserSelect) {
    const params = {
      source_path: `${basePath}/${userSelect.sourcePath}`,
      target_path: `${basePath}/${userSelect.targetPath}`,
    }
    const result: boolean = await invoke('cp_source_to_target', { params })
    if (result) {
      message.success('操作成功')
    } else {
      message.error('复制失败')
    }
  }

  return {
    treeData,
    loadTree,
    handleGoGO,
  }
}
