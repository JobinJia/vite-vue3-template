<script setup lang="tsx">
import type { MenuOption } from 'naive-ui'
import type { RouteRecordRaw } from 'vue-router'
import { NIcon } from 'naive-ui'
import { type Component, computed, h } from 'vue'
import { RouterLink, useRouter } from 'vue-router'

const router = useRouter()

function renderIcon(icon?: Component) {
  if (!icon)
    return null
  return () => h(NIcon, { size: 36 }, { default: () => h(icon, {
  }) })
}

const menuOptions = computed<MenuOption[]>(() => {
  const rootRoute = router.options.routes.find(r => r.path === '/') as RouteRecordRaw
  const routes = rootRoute?.children || []
  console.log(routes)

  return routes.map(r => ({
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: r.name,
          },
        },
        { default: () => r.meta!.title as string },
      ),
    key: r.name,
    icon: renderIcon(r.meta?.icon as Component),
  } as unknown as MenuOption))
})
</script>

<template>
  <n-layout has-sider>
    <n-layout-sider
      :collapsed-width="64"
      collapse-mode="width"
      collapsed
      class="h-screen backdrop-blur-sm bg-[url('@/assets/pagoda-8018757_1280.jpg')] bg-cover bg-center bg-no-repeat"
    >
      <n-menu
        collapsed
        class="h-full !bg-transparent"
        :options="menuOptions"
        key-field="key"
      />
    </n-layout-sider>
    <n-layout>
      <n-layout-content content-style="padding: 0;">
        <MRicePaper class="h-screen w-full">
          <router-view />
        </MRicePaper>
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<style scoped>
:deep(.n-menu-item-content) {
  padding-left: 20px !important;
}
</style>
