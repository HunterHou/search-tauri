<script setup lang="ts">
import { onMounted, reactive } from "vue";
// import VueMarkdownEditor from "@kangc/v-md-editor";
import "@kangc/v-md-editor/lib/style/base-editor.css";
// import vuepressTheme from "@kangc/v-md-editor/lib/theme/vuepress.js";
import { GetSettingInfo } from "@/api/setting";
import { SettingInfo } from "../settting";

const view = reactive({
  form: new SettingInfo(),
});
const loadData = async () => {
  const res = await GetSettingInfo();
  view.form = res;
};
onMounted(() => {
  loadData();
});
</script>
<template>
  <div style="text-align: left">
    <v-md-editor mode="preview" v-model="view.form.SystemHtml" height="700px"></v-md-editor>
  </div>
</template>
