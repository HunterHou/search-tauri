<template>
  <div class="topRef"> </div>
  <q-card class="q-dialog-plugin" style="width:100%;background-color: rgba(0, 0, 0, 0.1)">

    <div style="background-color: rgba(0, 0, 0, 0.8);white-space: nowrap;text-overflow: ellipsis;overflow: hidden;">
      <span class="q-mr-sm"
        style="-webkit-app-region: drag;color: rgb(213, 90, 90);font-weight: 550; font-size: medium;overflow: hidden">{{
          view.playing.Title }}</span>

    </div>
    <vue3VideoPlay v-show="view.playing?.Id" ref="vue3VideoPlayRef" id="vue3VideoPlayRef"
      style="object-fit: contain;width: 100%;height:auto;max-height: 99vh;" v-bind="optionsPC" @ended="playNext(1)" />
    <q-card-actions align="left">

      <q-btn flat style="color: #59d89d" :label="view.playing.Actress?.substring(0, 8)" @click="
        view.queryParam.Keyword = view.playing.Actress;
      fetchSearch();
      " />
      <q-btn flat style="color: goldenrod" :label="view.playing.Code?.substring(0, 8)" @click="
        view.queryParam.Keyword = view.playing.Code;
      fetchSearch();
      " />
      <q-input v-model="view.queryParam.Keyword" :dense="true" clearable @update:model-value="fetchSearch">
        <template v-slot:prepend>
          <q-icon name="ti-list" class="cursor-pointer">
            <q-popup-proxy>
              <div style="width: 200px;max-height: 50vh;">
                <q-list>
                  <q-item clickable v-ripple v-for="word in suggestions" :key="word"
                    @click="view.queryParam.Keyword = word; fetchSearch()">
                    <q-item-section>
                      <q-item-label>{{ word }}</q-item-label>
                    </q-item-section>
                  </q-item>
                </q-list>
              </div>
            </q-popup-proxy>
          </q-icon>
        </template>
        <template v-slot:append>
          <q-icon name="ti-search" title="搜" class="cursor-pointer" @click="fetchSearch">
          </q-icon>
        </template>
      </q-input>
      <q-btn color="red" label="关闭" @click="closeThis" />
      <q-btn-toggle v-model="view.queryParam.SortType" @update:model-value="fetchSearch()" toggle-color="primary"
        :options="[
          { label: '正', value: 'asc' },
          { label: '倒', value: 'desc' }
        ]" />
      <q-btn v-if="!view.playlist" color="primary" label="上一个" @click="playNext(-1)" />
      <q-btn v-if="!view.playlist" color="primary" label="下一个" @click="playNext(1)" />
      <q-btn color="orange" label="更多" @click="view.showMore = !view.showMore; fetchGetSettingInfo()" />
      <span v-if="view.showMore">
        <q-chip square color="red" text-color="white" v-for="tag in view.settingInfo.Tags" :key="tag"
          style="margin-left: 0px; padding: 0 4px">
          <span @click="view.queryParam.Keyword = tag; fetchSearch()">{{ tag }}</span>
        </q-chip>
      </span>

    </q-card-actions>
  </q-card>
  <div style="overflow: auto; background-color: rgba(0, 0, 0, 0.4)">
    <div class="row justify-center">
      <q-card class="q-ma-sm example-item" v-for="item in [...view.playList]" :key="item.Id">
        <q-img fit="cover" loading="lazy" draggable
          :src="convertFileSrc(item.Jpg || item.Png || item.Gif || 'public/icon.png')" class="item-img"
          @click="open(item)">
          <div style="
              padding: 0;
              margin: 0;
              background-color: rgba(0, 0, 0, 0);
              display: flex;
              flex-direction: row;
              justify-content: space-between;
              width: 100%;
            ">
            <div @click.stop="() => { }" style="
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                width: fit-content;
              ">

              <q-chip square color="red" size="sm" text-color="white" v-for="tag in item.Tags" :key="tag"
                style="margin-left: 0px; padding: 0 4px">
                <span @click="view.queryParam.Keyword = tag; fetchSearch()">{{ tag?.substring(0, 5) }}</span>
              </q-chip>
            </div>
            <q-chip @click.stop="() => { }" square size="sm" color="green" text-color="white"
              style="width: fit-content; margin-right: 0px; padding: 0 6px">
              <span> {{ item.MovieType }}</span>
            </q-chip>
          </div>
        </q-img>
        <q-card-section style="overflow: auto; padding: 4px">
          <div class="text-subtitle2" style="overflow: auto; padding: 0">
            <q-btn square color="red" size="sm" text-color="white" style="margin-left: 0px; padding: 0 4px">
              <span @click="deleteThis(item.Id)">删除</span>
            </q-btn>
            <q-chip @click.stop="() => { }" square size="sm" color="green" text-color="black" style="padding: 0px 4px">
              {{ item.SizeStr }}
            </q-chip>
            <span style="color: #031a0f;">
              {{ item.Title?.substring(0, 20) }}
            </span>
          </div>
        </q-card-section>
      </q-card>
    </div>
  </div>
</template>
<script setup>
import vue3VideoPlay from 'vue3-video-play';
import 'vue3-video-play/dist/style.css'; // 引入css
// import { useQuasar } from 'quasar';
import { computed, reactive, ref, watch } from 'vue';
import { useSystemProperty } from '../stores/System';
import { getFileStream } from './utils/images';
import { getPng } from './utils/images';
import { SearchAPI, DeleteFile, RefreshAPI } from './api/searchAPI';
import { GetSettingInfo } from './api/settingAPI';
import { useRouter } from 'vue-router';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const systemProperty = useSystemProperty();
const vue3VideoPlayRef = ref(null);
const { replace } = useRouter()

// const $q = useQuasar();
const view = reactive({
  playList: [],
  queryParam: { SortType: 'desc' },
  showMore: false,
  settingInfo: {},
  playing: {}
});

const props = defineProps({
  mode: {
    type: String,
    default: 'drawer'
  }
})

const deleteThis = async (Id) => {
  await DeleteFile(Id)
  await RefreshAPI()
  await fetchSearch()
}

const suggestions = computed(() => {
  return systemProperty.getSuggestions
})


const drawerRight = computed(() => {
  return systemProperty.drawerRight;
});

watch(drawerRight, (v) => {
  if (v) {
    fetchSearch();
  }
});


const open = (v) => {
  console.log('open', v)
  view.playing = v
  optionsPC.src = convertFileSrc(v.Id);
  optionsPC.webFullScreen = true;
  const top = document.querySelector('.topRef')
  if (top) {
    top.scrollTo(0, 0)
  }
  if (props.mode == 'page') {
    replace(`/playing?id=${v.Id}`)
  }
  setTimeout(() => {
    vue3VideoPlayRef.value && vue3VideoPlayRef.value.play();
  }, 100);
  if (!view.queryParam.Keyword) {
    view.queryParam.Keyword = v.Actress;
    fetchSearch();
  }
};

const fetchGetSettingInfo = async () => {
  const data = await GetSettingInfo();
  view.settingInfo = data;
};

const stop = () => {
  optionsPC.src = '';
};
const hideThis = () => {
  systemProperty.drawerRight = false;
};
const closeThis = () => {
  stop()
  if (props.mode == 'drawer') {
    hideThis();
  } else {
    window.close()
  }

};



const fetchSearch = async () => {
  const data = await SearchAPI({
    ...systemProperty.FileSearchParam,
    ...view.queryParam,
    FileType: view.settingInfo.VideoTypes || ['mp4', 'mkv'],
    Page: 1,
    PageSize: 500,
  });
  view.playList = [...data.Data];
  if (!view.playing) {
    view.playing = view.playList[0];
    open(view.playing)
  }
};


const playNext = (step) => {
  if (!view.playList) {
    return;
  }
  for (let i = 0; i < view.playList.length; i++) {
    if (view.playList[i].Id === view.playing.Id) {
      if (i == view.playList.length - 1) {
        open(view.playList[0])
        return;
      } else {
        if (i + step <= 0) {
          open(view.playList[0])
          return;
        } else {
          open(view.playList[i + step])
          return;
        }
      }
    }
  }
};

const optionsPC = reactive({
  width: '100%', //播放器高度
  height: 'auto', //播放器高度
  color: '#409eff', //主题色
  title: view.playing?.Title, //视频名称
  src: '', //视频源 convertFileSrc(view.playing.Path)
  muted: false, //静音
  preload: 'false',
  webFullScreen: false,
  speedRate: ['1.0', '1.25', '1.5', '2.0'], //播放倍速
  autoPlay: false, //自动播放
  loop: false, //循环播放
  mirror: false, //镜像画面
  ligthOff: true, //关灯模式
  currentTime: 1,
  volume: systemProperty.videoOptions?.volume, //默认音量大小
  control: systemProperty.videoOptions?.control, //是否显示控制
  controlBtns: systemProperty.videoOptions?.controlBtns, //显示所有按钮,
});

defineExpose({
  open,
  stop,
});


</script>
<style lang="scss" scoped>
.example-item {
  width: 200px;
  height: auto;
  max-height: 440px;
  overflow: hidden;
}

.item-img {
  width: 200px;
  height: auto;
  max-height: 240px;
}
</style>
