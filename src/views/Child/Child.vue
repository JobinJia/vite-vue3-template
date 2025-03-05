<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  userId: string
}>()

const count = computed(() => {
  return Number(props.userId)
})

const doubleCount = computed(() => count.value * 2)
</script>

<template>
  <div>
    child 页面
    <h3 />
    <p>
      userId ===> {{ userId }}
    </p>
    <p>
      component ===>
    </p>
    <div class="color-red">
      <p>count => {{ count }}</p>
      <p>doubleCount => {{ doubleCount }}</p>
    </div>
  </div>
</template>

<style scoped>

</style>
