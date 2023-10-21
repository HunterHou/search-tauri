<template>
  <div>
    <q-banner inline-actions rounded class="bg-blue text-white q-mt-sd" style="position: relative">
      <div class="text-h5 q-pa-md">文件扫描分析</div>
      <template v-slot:action>
        <q-btn style="background: #ff0080; color: white" label="扫描" icon="search" :loading="indexLoading" size="small"
          @click="refreshIndex()" />
        <q-btn color="green" class="q-ml-md" label="刷新" icon="refresh" @click="f5" />
      </template>
    </q-banner>
    <div v-if="tableData.length == 0 && tagData.length == 0 && scanTime.length == 0">
      <h3>数据分析中ing</h3>
    </div>
    <div class="main" v-else>
      <q-splitter v-model="splitterModel">
        <template v-slot:before>
          <div class="q-pa-md">
            <div class="text-h5 q-mb-md">标签分布</div>
            <div class="q-gutter-sm" style="
              display: flex;
              flex-direction: row;
              flex-wrap: wrap;
              justify-content: flex-start;
            ">
              <div v-for="tag in tagData" :key="tag" style="width: auto">
                <q-btn color="secondary" class="btn-fixed-width" v-if="tag.Cnt > 1" @click="folderGotoMenu(tag.Name)">
                  {{ `${tag.Name} (${tag.Cnt})` }}
                  <q-badge color="orange" floating>{{ tag.SizeStr }}</q-badge>
                </q-btn>
              </div>
            </div>
          </div>
        </template>

        <template v-slot:after>
          <q-splitter v-model="insideModel" horizontal>
            <template v-slot:before>
              <div class="q-pa-md">
                <div class="text-h5 q-mb-md">扫描分析</div>
                <div class="row justify-center q-gutter-sm">
                  <q-card class="my-card" v-for="sc in scanTime" :key="sc">
                    <q-card-section>
                      <div class="text-h6">{{ sc.Name }}</div>
                      <div class="text-subtitle2">时间：{{ sc.Cnt }}</div>
                      <div class="text-subtitle2">文件数：{{ sc.Size }}</div>
                    </q-card-section>
                    <q-separator />
                    <q-card-actions vertical>
                      <q-btn flat color="blue" @click="gotoMenu({ ...sc, IsDir: true })">传送</q-btn>
                    </q-card-actions>
                  </q-card>
                </div>
              </div>
            </template>

            <template v-slot:after>
              <div class="q-pa-md">
                <div class="text-h5 q-mb-md">文件分析</div>
                <div class="row justify-center q-gutter-sm">
                  <q-card class="my-card" v-for="sc in tableData" :key="sc">
                    <q-card-section>
                      <div class="text-h6">{{ sc.Name }}</div>
                      <div class="text-subtitle2">文件数：{{ sc.Cnt }}</div>
                      <div class="text-subtitle2">文件大小：{{ sc.SizeStr }}</div>
                    </q-card-section>
                    <q-separator />
                    <q-card-actions vertical>
                      <div style="
                        display: flex;
                        flex-direction: row;
                        justify-content: space-around;
                      ">
                        <q-btn color="blue" flat v-if="!sc.IsDir" @click="gotoMenu(sc)">传送</q-btn>
                        <q-btn color="blue" flat v-if="sc.IsDir" @click="openThis(sc)">打开</q-btn>
                        <q-btn color="red" flat v-if="sc.IsDir" @click="deleteThis(sc)">删除</q-btn>
                      </div>
                    </q-card-actions>
                  </q-card>
                </div>
              </div>
            </template>
          </q-splitter>
        </template>
      </q-splitter>
    </div>

  </div>
</template>

<script setup>
import { useQuasar } from 'quasar';
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
const splitterModel = ref(50);
const insideModel = ref(50);

import {
  RefreshAPI,
  OpenFolerByPath,
  DeleteFolerByPath,
} from '../components/api/searchAPI';
import { TypeSizeMap, TagSizeMap, ScanTime } from '../components/api/homeAPI';
import { onKeyStroke } from '@vueuse/core';
import { useSystemProperty } from '../stores/System';
import { appWindow } from '@tauri-apps/api/window';

const { push } = useRouter();
const systemProperty = useSystemProperty();
document.title = '分析';
appWindow?.setTitle('分析')

const $q = useQuasar();
const indexLoading = ref(false);
const tableData = ref([]);
const tagData = ref([]);
const scanTime = ref([]);

onKeyStroke(['`'], () => {
  refreshIndex();
});

const folderGotoMenu = (Name) => {
  systemProperty.setPage(1);
  systemProperty.FileSearchParam.Keyword = Name;
  systemProperty.setMovieType('');
  push('/search?from=index');
};

const gotoMenu = (data) => {
  const { IsDir, Name } = data;
  const movieType = !IsDir && Name !== '全部' ? Name : '';
  systemProperty.setPage(1);
  if (IsDir) {
    systemProperty.setKeyword(Name);
  }
  systemProperty.setMovieType(movieType);
  push('/search?from=index');
};
const loadTypeSize = async () => {
  const res = await TypeSizeMap();
  if (res) {
    tableData.value = res;
    loadTagSize();
    loadScanTime();
  }
};

const loadTagSize = async () => {
  const res = await TagSizeMap();
  if (res) {
    tagData.value = res;
  }
};
const loadScanTime = async () => {
  const res3 = await ScanTime();
  scanTime.value = res3;
};
onMounted(() => {
  loadTypeSize();
});

const openThis = async (data) => {
  const { Name } = data;
  const res = await OpenFolerByPath({ dirpath: Name });
  if (res.Code == 200) {
    $q.notify({ type: 'positive', message: '执行成功', multiLine: true, position: 'bottom-right' });
  } else {
    $q.notify({ type: 'warning', message: '执行失败', multiLine: true, position: 'bottom-right' });
  }
};
const deleteThis = async (data) => {
  const { Name } = data;
  const res = await DeleteFolerByPath({ dirpath: Name });
  if (res.Code == 200) {
    $q.notify({ type: 'positive', message: '执行成功', multiLine: true, position: 'bottom-right' });
    refreshIndex();
  } else {
    $q.notify({ type: 'warning', message: '执行失败', multiLine: true, position: 'bottom-right' });
  }
};
const refreshIndex = async () => {
  const { data } = await RefreshAPI();
  console.log(data);
  if (data.Code == 200) {
    f5();
  } else {
    $q.notify({ type: 'warning', message: data.Message });
  }
};

const f5 = () => {
  window.location.reload();
};
</script>
