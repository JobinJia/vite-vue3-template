<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

function handleClick(userId: string) {
  console.log(userId)
  if (userId === 'home') {
    router.push('/home')
  } else {
    router.push(`/child/${userId}`)
  }
}
</script>

<template>
  <div class="w-50 flex flex-col gap-3">
    <button
      class="flex w-auto p-2 text-4 border-hidden bg-cyan border-rd-1 outline-hidden cursor-pointer hover-bg-emerald hover-text-white"
      @click="handleClick('home')"
    >
      Home
    </button>
    <button
      class="flex w-auto p-2 text-4 border-hidden bg-cyan border-rd-1 outline-hidden cursor-pointer hover-bg-emerald hover-text-white"
      @click="handleClick('11')"
    >
      User1
    </button>
    <button
      class="flex w-auto p-2 text-4 border-hidden bg-cyan border-rd-1 outline-hidden cursor-pointer hover-bg-emerald hover-text-white"
      @click="handleClick('22')"
    >
      User2
    </button>
    <button
      class="flex w-auto p-2 text-4 border-hidden bg-cyan border-rd-1 outline-hidden cursor-pointer hover-bg-emerald hover-text-white"
      @click="handleClick('33')"
    >
      User3
    </button>
  </div>
  <div>
    <RouterView />
  </div>
</template>
