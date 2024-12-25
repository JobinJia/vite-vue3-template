<script setup lang="ts">
import { ref } from 'vue'

const v = ref(false)
const c = ref('')

function handleClick() {
  if (v.value) {
    c.value = 'true'
  }
  c.value = 'false'
}
</script>

<template>
  <button @click="handleClick" />
  <RouterView />
</template>
