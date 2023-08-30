import { URL, fileURLToPath } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import unoCSS from 'unocss/vite'
import icons from 'unplugin-icons/vite'

import autoComponents from 'unplugin-vue-components/vite'

// import {
//   AntDesignVueResolver,
//   NaiveUiResolver,
// } from 'unplugin-vue-components/resolvers'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    unoCSS(),
    icons({
      autoInstall: true,
      compiler: 'vue3',
    }),
    autoComponents({
      dts: true,
      dirs: ['src/components'],
      extensions: ['vue', 'tsx'],
      // resolvers: [
      // AntDesignVueResolver(),
      // NaiveUiResolver(),
      // ],
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
})
