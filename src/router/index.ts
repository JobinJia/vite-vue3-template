import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Root',
      redirect: '/home',
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('@/views/Login/index.vue'),
    },
    {
      path: '/home',
      name: 'Home',
      component: () => import('@/views/Home/HomeView.vue'),
    },
  ],
})

export default router
