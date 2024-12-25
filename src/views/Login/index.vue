<script setup lang="ts">
import { useRequest } from '@jobin/request'
import { LoginApi, type LoginRequest } from '@jobin/request/api'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const state = ref<LoginRequest>({
  account: 'name',
  password: 'password',
  validateCode: '',
})

const { loading, send, data } = useRequest(
  () => LoginApi.login(state.value),
  {
    immediate: false,
  },
)
function handleLogin() {
  send()
}
const router = useRouter()
function go() {
  router.push('/')
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
    <hr>
    <p>
      <code>
        {{ JSON.stringify(data, null, 2) }}
      </code>
    </p>
    <button class="w-20 h-10 bg-fuchsia hover:shadow-blue" @click="go">
      GOGO
    </button>
  </div>
</template>
