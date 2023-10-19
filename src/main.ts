import { createApp} from 'vue'
import './style.css'
import Main from './App.vue'

import {StoreSetup} from './store/pinia'
import {RouterSetup} from './route'
import {ElementSetup} from './plugin/element'

// import Vue3videoPlay from "vue3-video-play"; // 引入组件
import "vue3-video-play/dist/style.css"; // 引入css

// import VMdEditor from '@kangc/v-md-editor';
// import '@kangc/v-md-editor/lib/style/base-editor.css';
// import githubTheme from '@kangc/v-md-editor/lib/theme/github.js';
// import '@kangc/v-md-editor/lib/theme/style/github.css';
// import VMdPreviewHtml from '@kangc/v-md-editor/lib/preview-html';
// import '@kangc/v-md-editor/lib/style/preview-html.css';

// 引入使用主题的样式
// import '@kangc/v-md-editor/lib/theme/style/vuepress';

// highlightjs
// import hljs from 'highlight.js';

// VMdEditor.use(githubTheme, {
//     Hljs: hljs,
//   });

const AppInit = (app:any) => {
    RouterSetup(app)
    StoreSetup(app)
    ElementSetup(app)
}

const app = createApp(Main)
AppInit(app)
// app.use(Vue3videoPlay)
// app.use(VMdEditor);
// app.use(VMdPreviewHtml);
app.mount('#app')
