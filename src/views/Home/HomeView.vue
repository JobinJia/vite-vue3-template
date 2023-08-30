<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useCounterStore } from '@/stores/counter'
import MaterialSymbolsAdd from '~icons/material-symbols/add'

const counter = useCounterStore()

const { doubleCount, count } = storeToRefs(counter)
</script>

<template>
  <main>
    <h2 class="w-full flex">
      Home Page
    </h2>
    <button class="w-auto flex p-2 text-4 border-hidden bg-cyan border-rd-1 outline-hidden cursor-pointer hover-bg-emerald hover-text-white" @click="counter.increment">
      <MaterialSymbolsAdd />
      Increment
    </button>
    <p>count => {{ count }}</p>
    <p>doubleCount => {{ doubleCount }}</p>
  </main>
</template>
