
import './styles/quasar.scss'
import '@quasar/extras/material-icons/material-icons.css'
import quasarLang from 'quasar/lang/zh-CN'
import {
  Dialog,
  Notify
} from 'quasar'

// To be used on app.use(Quasar, { ... })
export default {
  config: {},
  lang: quasarLang,
  plugins: {
    Notify,Dialog
  }
}