import { createApp } from 'vue'
import './assets/index.css'
import App from './App.vue'


import { createMemoryHistory, createRouter } from 'vue-router'
import HomeView from '@/views/Home.vue'
import ServicesView from '@/views/services.vue'
const routes = [
  { path: '/', component: HomeView },
  { path: '/services', component: ServicesView },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App)
  .use(router)
  .mount('#app')
