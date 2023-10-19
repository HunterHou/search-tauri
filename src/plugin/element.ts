
import elementPlus from 'element-plus'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import { App } from 'vue'

export const ElementSetup=(app:App<Element>)=>{
    app.use(elementPlus, {
        size: 'small',
        locale: zhCn
    })
    for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
        app.component(key, component)
    }
}
