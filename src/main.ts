import '@quasar/extras/material-icons/material-icons.css'
import 'quasar/src/css/index.sass'

import App from './App.vue'
import { createApp } from 'vue'
import { Quasar } from 'quasar'
import quasarUserOptions from './quasar-user-options'
import route from './router'

const myApp = createApp(App)
myApp.use(Quasar, quasarUserOptions)
myApp.use(route)
myApp.mount('#app')
