import '@quasar/extras/material-icons/material-icons.css'
import 'quasar/src/css/index.sass'

import App from './App.vue'
import { createApp } from 'vue'
import { Quasar } from 'quasar'
import quasarUserOptions from './quasar-user-options'
import router from './router'

const myApp = createApp(App)
myApp.use(Quasar, quasarUserOptions)
myApp.use(router)
myApp.mount('#app')
