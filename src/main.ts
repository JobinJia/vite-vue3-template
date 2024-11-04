import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './App.vue'

import router from './router'
import './assets/main.css'

import 'virtual:uno.css'
import 'the-new-css-reset/css/reset.css'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
