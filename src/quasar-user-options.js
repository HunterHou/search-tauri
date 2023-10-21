
import './styles/quasar.scss'
import '@quasar/extras/material-icons/material-icons.css'
import quasarLang from 'quasar/lang/zh-CN'

// To be used on app.use(Quasar, { ... })
export default {
  config: {},
  lang: quasarLang,
  extras: [
    'material-icons',
    'mdi-v6',
    'ionicons-v4', // 最后一个webfont在v4.6.3中可用。
    'eva-icons',
    'fontawesome-v6',
    'themify',
    'line-awesome',
    'bootstrap-icons'
  ],
  plugins: {
  }
}