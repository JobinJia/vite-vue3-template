<script setup lang="ts">
import { NConfigProvider, NMessageProvider } from 'naive-ui'
</script>

<template>
  <m-rice-paper class="h-screen w-full">
    <NConfigProvider>
      <NMessageProvider>
        <router-view />
      </NMessageProvider>
    </NConfigProvider>
  </m-rice-paper>
</template>
