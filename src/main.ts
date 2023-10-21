import '@quasar/extras/material-icons/material-icons.css'
import 'quasar/src/css/index.sass'

import App from './App.vue'
import { createApp } from 'vue'
import { Quasar } from 'quasar'
import quasarUserOptions from './quasar-user-options'
import router from './router'
import { createPinia } from 'pinia'

const myApp = createApp(App)
const pinia = createPinia()
myApp.use(router())
myApp.use(pinia)
myApp.use(Quasar, quasarUserOptions)
myApp.mount('#app')
