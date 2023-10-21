<template>
  <div>
    <q-layout view="LHR lpr lfr" container style="height: 100vh" class="shadow-2 rounded-borders">
      <q-header reveal class="bg-black">
        <q-toolbar>
          <q-btn flat @click="drawerLeft = !drawerLeft" round dense icon="menu" />
          <q-toolbar-title style="-webkit-app-region: drag">
            文件搜索<q-btn dense flat icon="refresh" @click="refreshThis">
            </q-btn>
          </q-toolbar-title>
          <EssentialLink v-for="link in essentialLinks" :key="link.title" v-bind="link" v-show="isWideScreen" :style="{
            color: currentPath == link.link ? 'red' : '',
            scale: 1.2,
          }" />
          <q-btn dense flat color="red" v-if="shutdownLeftSecond">关机倒计时：{{ shutdownLeftSecond }}</q-btn>
          <q-space />
          <q-btn color="green" flat @click="systemProperty.drawerRight = !systemProperty.drawerRight" round dense
            icon="menu">
            <span v-if="systemProperty.drawerRight">{{
              `${(systemProperty &&
                systemProperty.Playing?.Code?.substring(0, 8)) ||
                systemProperty.Playing?.Title?.substring(0, 8)
                }`
            }}</span>
          </q-btn>
          <q-btn dense flat icon="ti-timer" @click="confirmShutDown" />

          <!-- <q-bar class="bg-black text-white">
             <q-btn dense flat icon="minimize"  />
           <q-btn dense flat icon="crop_square" @click="maxMainWindow" />
            <q-btn dense flat icon="close" @click="confirmClose" />
            <q-btn dense flat icon="ti-timer" @click="confirmDelete" />
          </q-bar>-->
        </q-toolbar>
      </q-header>

      <q-drawer v-model="drawerLeft" :width="200" :breakpoint="700" bordered class="bg-grey-3">
        <q-scroll-area class="fit">
          <q-list>
            <q-item-label header> 你的搜索工具 </q-item-label>
            <EssentialLink v-for="link in essentialLinks" :key="link.title" v-bind="link" :style="{
              color: currentPath == link.link ? 'red' : '',
              scale: 1.2,
            }" />
          </q-list>
        </q-scroll-area>
      </q-drawer>
      <q-drawer side="right" :width="isWideScreen ? 750 : 420" v-model="systemProperty.drawerRight" bordered
        class="bg-grey-3">
        <Playing ref="vue3VideoPlayRef" mode="drawer" />
      </q-drawer>
      <q-page-container>
        <router-view />
      </q-page-container>
    </q-layout>
    <ShutdownComponent ref="shutdown" />
  </div>
</template>

<script setup>
import { computed, ref, watch } from 'vue';
import Playing from '@/components/PlayingVideo.vue';
import { useSystemProperty } from '@/stores/System';
import { useQuasar } from 'quasar';
import EssentialLink from '@/components/EssentialLink.vue';
import ShutdownComponent from '@/components/ShutdownComponent.vue';

import { useRoute } from 'vue-router';
const systemProperty = useSystemProperty();
const $q = useQuasar();

const shutdown = ref(null);

const isWideScreen = computed(() => {
  return $q.screen.width > 1000;
});

const playing = computed(() => {
  return systemProperty.Playing || {};
});

const shutdownLeftSecond = computed(() => {
  let left = systemProperty.shutdownLeftSecond;
  if (!left) {
    return null;
  }

  return (
    `${Math.floor(systemProperty.shutdownLeftSecond / 3600)} 时 ${Math.floor((systemProperty.shutdownLeftSecond / 60) % 60)} 分 ${Math.floor(systemProperty.shutdownLeftSecond % 60)} 秒`
  );
});

const vue3VideoPlayRef = ref(null);

watch(playing, (v) => {
  if (v && v.Id) {
    vue3VideoPlayRef.value && vue3VideoPlayRef.value.open(v);
  } else {
    vue3VideoPlayRef.value &&  vue3VideoPlayRef.value.stop();
  }
});

const drawerLeft = ref(false);

const currentPath = computed(() => {
  return useRoute().path;
});

const refreshThis = () => {
  window.location.reload();
};

// const closeWindow = () => {
//   window.close()
// }

// const maxMainWindow = () => {
//   window.electron.maxMainWindow()
// }

// const hideMainWindow = () => {
//   window.electron.hideMainWindow()
// }

// const confirmClose = () => {
//   $q.dialog({
//     message: '确定关闭吗?',
//     cancel: true,
//     persistent: true
//   })
//     .onOk(() => {
//       console.log('>>>> onOk');
//       closeWindow()
//     })
//     .onCancel(() => {
//       console.log('>>>> Cancel');
//     })
//     .onDismiss(() => {
//       // console.log('I am triggered on both OK and Cancel')
//     });
// };

const confirmShutDown = () => {
  shutdown.value.open();
};

const essentialLinks = [
  {
    title: '首页',
    caption: 'quasar.dev',
    icon: 'home',
    link: '/',
  },
  {
    title: '搜索',
    caption: 'github.com/quasarframework',
    icon: 'search',
    link: '/search',
  },
  {
    title: '图鉴',
    caption: 'chat.quasar.dev',
    icon: 'image',
    link: '/picture',
  },
  {
    title: '设置',
    caption: 'chat.quasar.dev',
    icon: 'settings',
    link: '/setting',
  },
  {
    title: '系统',
    caption: 'forum.quasar.dev',
    icon: 'chat',
    link: '/system',
  },
];
</script>
