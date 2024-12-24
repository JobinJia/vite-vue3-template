<script setup lang="ts">
import { useRequest } from '@jobin/request'
import { LoginApi, type LoginRequest } from '@jobin/request/api'
import { ref } from 'vue'

const state = ref<LoginRequest>({
  account: '',
  password: '',
  validateCode: '',
})

const { loading, send } = useRequest(
  () => LoginApi.login(state.value),
  {
    immediate: false,
  },
)
function handleLogin() {
  send()
}
</script>

<template>
  <div>
    {{ loading }}
    <input v-model="state.account" type="text">
    <input v-model="state.password" type="password">
    <button @click="handleLogin">
      Login
    </button>
  </div>
</template>
