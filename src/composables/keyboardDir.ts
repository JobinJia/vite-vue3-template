import { open } from '@tauri-apps/plugin-dialog'
import { useBasePath } from './basePath'

export function useKeyboardDir() {
  const { basePath } = useBasePath()

  async function handleSetDir() {
    if (basePath.value) {
      return
    }

    const path = await open({
      multiple: false,
      directory: true,
    })

    if (path) {
      basePath.value = path
    }
  }

  function handleResetDir() {
    basePath.value = ''
  }

  return {
    handleSetDir,
    handleResetDir,
  }
}
