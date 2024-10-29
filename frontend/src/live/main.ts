import { createPinia } from 'pinia'
import { createApp } from 'vue'

import '../style.css'
import App from './LiveApp.vue'

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.mount('#app')
