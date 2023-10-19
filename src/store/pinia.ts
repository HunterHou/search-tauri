import {createPinia} from 'pinia'
import {App} from 'vue'
import piniaPluginPersist from 'pinia-plugin-persist'

const store = createPinia()
export function StoreSetup(app : App < Element >) {
    store.use(piniaPluginPersist)
    app.use(store)
}
export {
    store
}
