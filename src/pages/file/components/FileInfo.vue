<template>
  <q-dialog ref="dialogRef" @hide="onDialogClose" style="width: 80vw !important;" v-model:model-value="showDialog">
    <q-card class="q-dialog-plugin q-pa-md" :style="{
      color: 'white',
      height: '100%',
      width: '100%',
      padding: '0 4px',
      lineHeight: '32px',
      maxWidth: '80vw !important'
    }">
      <div
        style="position: relative;margin-top: 0;display: flex;flex-direction: row;flex-wrap:nowrap;justify-content: space-between;overflow: hidden;">
        <div>
          <q-btn class="q-mr-sm" size="sm" v-if="showDetail != 'movie'" color="red" @click="showMovie"
            icon="ti-arrow-circle-right">播放</q-btn>
          <q-btn class="q-mr-sm" size="sm" v-if="showDetail != 'detail'" color="purple" @click="showDetail = 'detail'"
            icon="ti-arrow-circle-right">详情</q-btn>
          <q-btn class="q-mr-sm" size="sm" v-if="showDetail != 'image'" color="deep-orange" @click="showDetail = 'image'"
            icon="ti-arrow-circle-right">图层</q-btn>
          <q-btn class="q-mr-sm" size="sm" v-if="showDetail != 'web'" color="deep-orange" @click="showDetail = 'web'"
            icon="ti-world">JavBus</q-btn>
        </div>
        <div><q-btn class="q-mr-sm" size="sm" ripple color="green" icon="ti-fullscreen"
            @click="openPlay(view.item)">大屏</q-btn>
          <q-btn class="q-mr-sm" size="sm" ripple color="green-7" icon="ti-blackboard"
            @click="openDialog(view.item)">侧屏</q-btn>
          <q-btn class="q-mr-sm" size="sm" color="primary" icon="ti-control-eject"
            @click="commonExec(PlayMovie(view.item.Id))">播放</q-btn>
          <q-btn class="q-mr-sm" size="sm" color="primary" icon="open_in_new"
            @click="commonExec(OpenFileFolder(view.item.Id))">文件夹</q-btn>
          <q-btn class="q-mr-sm" size="sm" color="secondary" icon="ti-import"
            @click="commonExec(DownImageList(view.item.Id))">挂图</q-btn>
          <q-btn class="q-mr-sm" size="sm" color="amber" glossy text-color="black" icon="ti-trash"
            @click="confirmDelete(view.item)">删除</q-btn>
          <q-btn class="q-mr-sm" size="sm" color="black" @click="moveThis(view.item)" icon="ti-control-shuffle">移动</q-btn>
          <q-btn class="q-mr-sm" size="sm" ripple color="red" icon="ti-close" @click="onDialogClose">关闭</q-btn>
        </div>
      </div>

      <div style="margin-top: 0;height: 96%;overflow: auto;">
        <div v-if="showDetail == 'web'" style="overflow: auto;">
          <iframe :frameborder="0" :allowfullscreen="true" width="100%" height="900px"
            :src="`${view.settingInfo.BaseUrl}${view.item.Code}`"></iframe>
        </div>
        <div v-if="showDetail == 'movie'">
          <Playing ref="vue3VideoPlayRef" mode="drawer" />
        </div>
        <div v-if="showDetail == 'detail'">
          <q-img fit="fit" easier draggable :src="getJpg(view.item.Id)" style="min-width:600px ;max-height: 50vh">
          </q-img>
          <q-field label="Code" stack-label>
            <template v-slot:control>
              <div class="self-center full-width no-outline cursor-pointer" style="color: blue" tabindex="0"
                @click="searchCode(view.item)">
                {{ view.item.Code }}
              </div>
            </template>
          </q-field>
          <q-field label="Actress" stack-label>
            <template v-slot:control>
              <div class="self-center full-width no-outline" tabindex="0">
                {{ view.item.Actress }}
              </div>
            </template>
          </q-field>
          <q-field label="Name" stack-label>
            <template v-slot:control>
              <div class="self-center full-width no-outline" tabindex="0">
                {{ formatTitle(view.item.Name) }}
              </div>
            </template>
          </q-field>
          <q-field label="Time" stack-label>
            <template v-slot:control>
              <div class="self-center full-width no-outline" tabindex="0">
                {{ formatTitle(view.item.MTime) }}
              </div>
            </template>
          </q-field>
          <q-field label="Path" stack-label>
            <template v-slot:control>
              <div class="self-center full-width no-outline" tabindex="0">
                {{ view.item.Path }}
              </div>
            </template>
          </q-field>
        </div>
        <div v-if="showDetail == 'image'">
          <q-img fit="fit" v-for="item in view.prewiewImages" :key="item.Id" :src="getTempImage(item.Id)"
            style="width: 100%;height: auto;"></q-img>
        </div>
      </div>
    </q-card>
  </q-dialog>
</template>
<script setup>
import { useQuasar } from 'quasar'
import { useDialogPluginComponent } from 'quasar';
import { onMounted, reactive, ref } from 'vue';

import { formatTitle } from '@/components/utils';
import { GetSettingInfo } from '@/components/api/settingAPI';
import {
  QueryDirImageBase64, OpenFileFolder,
  DownImageList, FileRename, DeleteFile,
  PlayMovie
} from '@/components/api/searchAPI';
import { getJpg, getTempImage } from '@/components/utils/images';
import { useSystemProperty } from '@/stores/System';
import Playing from '@/components/PlayingVideo.vue';

const $q = useQuasar()
const systemProperty = useSystemProperty();

const vue3VideoPlayRef = ref(null)
const showDialog = ref(false)
const showDetail = ref('detail')

const commonExec = async (exec) => {
  const { Code, Message } = (await exec) || {};
  if (Code != 200) {
    $q.notify({ type: 'negative', message: `${Message}`, multiLine: true, position: 'bottom-right' });
  }
}

import { WebviewWindow } from '@tauri-apps/api/window'

const openPlay = (item) => {
  const url = `#/playing/${item.Id}`
  // if ($q.platform.is.electron) {
  //   window.electron.createWindow({ router: url })
  // } else {
  //   window.open(url)
  // }
  const webview = new WebviewWindow("player", {
    title: item.Name,
    focus: true,
    skipTaskbar: true,
    width: 1200,
    height: 800,
    url
  })
  webview.once('tauri://created', function () {
    console.log("tauri://created")
  })

}

const showMovie = () => {
  showDetail.value = 'movie';
  setTimeout(() => {
    vue3VideoPlayRef.value && vue3VideoPlayRef.value.open(view.item)
  }, 100);
}

const moveThis = async (item) => {
  const res = await FileRename({ ...item, NoRefresh: true, MoveOut: true });
  if (res.Code == 200) {
    $q.notify({ type: 'negative', message: res.Message, multiLine: true, position: 'bottom-right' });
  } else {
    $q.notify({ type: 'negative', message: res.Message, multiLine: true, position: 'bottom-right' });
  }
};


const confirmDelete = (item) => {
  $q.dialog({
    title: item.Name,
    message: '确定删除吗?',
    cancel: true,
    persistent: true
  })
    .onOk(() => {
      commonExec(DeleteFile(item.Id), true);
    })
    .onCancel(() => {
      console.log('>>>> Cancel');
    })
    .onDismiss(() => {
      // console.log('I am triggered on both OK and Cancel')
    });
};

const openDialog = (item) => {
  view.currentData = item;
  systemProperty.Playing = item;
  systemProperty.drawerRight = true;
};

const view = reactive({
  item: {},
  settingInfo: {},
  callback: null,
  prewiewImages: [],
});

defineEmits([
  // REQUIRED; 需要明确指出
  // 组件通过 useDialogPluginComponent() 暴露哪些事件
  ...useDialogPluginComponent.emits,
]);

const open = (data) => {
  const { item, cb, playing } = data
  view.prewiewImages = [];
  view.item = { ...item };
  view.callback = cb;
  dialogRef.value.show();
  if (playing) {
    showMovie()
  }
  setTimeout(() => {
    QueryDirImageBase64(item.Id).then((res) => {
      view.prewiewImages = res.data
    })
  }, 500);
};

const fetchSetting = async () => {
  const res = await GetSettingInfo();
  view.settingInfo = res.data;
};

const searchCode = (item) => {
  const url = `${view.settingInfo.BaseUrl}/${item.Code}`
  console.log(url)
  if ($q.platform.is.electron) {
    window.electron.createWindow({ router: url, width: 1280, height: 1000, titleBarStyle: '', })
  } else {
    window.open(url)
  }
};

// onDialogOK, onDialogCancel
const { dialogRef, onDialogHide } = useDialogPluginComponent();

const onDialogClose = () => {
  showDetail.value = 'detail'
  vue3VideoPlayRef.value && vue3VideoPlayRef.value.stop()
  showDialog.value = false
  onDialogHide()
}
// dialogRef      - 用在 QDialog 上的 Vue ref 模板引用
// onDialogHide   - 处理 QDialog 上 @hide 事件的函数
// onDialogOK     - 对话框结果为 ok 时会调用的函数
//                    示例: onDialogOK() - 不带参数
//                    示例: onDialogOK({ /*.../* }) - 带参数
// onDialogCancel - 对话框结果为 cancel 时调用的函数

// 这是示例的内容，不是必需的
// const onOKClick = () => {
// REQUIRED！ 对话框的结果为 ok 时，必须调用 onDialogOK()  (参数是可选的)
// onDialogOK()
// 带参数的版本: onDialogOK({ ... })
// ...会自动关闭对话框
// }

onMounted(() => {
  fetchSetting();
});

defineExpose({
  open,
});
</script>
