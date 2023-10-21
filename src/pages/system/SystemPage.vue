<template>
  <div>
    <span>系统信息：</span>
    <span>{{ $q.platform.is }}</span>
  </div>
  <div class="q-pa-md SystemHtml" v-html="view.settingInfo.SystemHtml"></div>
</template>

<script setup>
import {onMounted, reactive} from 'vue';
import {GetSettingInfo} from '../../components/api/settingAPI';

const view = reactive({
  settingInfo: {},
});

const fetchSearch = async () => {
  const {data} = await GetSettingInfo();
  console.log(data);
  view.settingInfo = data;
};
import { appWindow } from '@tauri-apps/api/window';
onMounted(() => {
  document.title = '系统信息'
  appWindow?.setTitle('系统信息')
  fetchSearch();
});
</script>
<style lang="scss" scoped>
.SystemHtml {
  padding: 1rem;
  margin: 0;
}
</style>
