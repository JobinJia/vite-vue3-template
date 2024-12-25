/// <reference types="vite/client" />

interface ImportMetaEnv {
  // 在这里添加其他环境变量
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
