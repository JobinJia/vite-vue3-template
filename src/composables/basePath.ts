import { useStorage } from '@vueuse/core'

export function useBasePath() {
  const basePath = useStorage('basePath', '')

  const setBasePath = (path: string) => {
    basePath.value = path
  }

  return {
    basePath,
    setBasePath,
  }
}
