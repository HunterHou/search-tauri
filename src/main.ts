import { createApp } from "vue";
import "./style.css";
import Main from "./App.vue";

import { StoreSetup } from "./store/pinia";
import { RouterSetup } from "./route";
import { ElementSetup } from "./plugin/element";

import Vue3videoPlay from "vue3-video-play"; // 引入组件
import "vue3-video-play/dist/style.css"; // 引入css

import VueMarkdownEditor from '@kangc/v-md-editor';
import '@kangc/v-md-editor/lib/style/base-editor.css';
import vuepressTheme from '@kangc/v-md-editor/lib/theme/vuepress.js';
import '@kangc/v-md-editor/lib/theme/style/vuepress.css';

import Prism from 'prismjs';

VueMarkdownEditor.use(vuepressTheme, {
  Prism,
});

const AppInit = (app: any) => {
  RouterSetup(app);
  StoreSetup(app);
  ElementSetup(app);
};

const app = createApp(Main);
AppInit(app);
app.use(Vue3videoPlay)
app.use(VueMarkdownEditor)
app.mount("#app");
