import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('@/layouts/MainLayout.vue'),
    children: [
      { path: '/', component: () => import('@/pages/IndexPage.vue') },
      { path: '/search', component: () => import('@/pages/file/SearchPage.vue') },
      {
        path: '/picture',
        component: () => import('@/pages/picture/PicturePage.vue'),
      },
      {
        path: '/setting',
        component: () => import('@/pages/setting/SettingPage.vue'),
      },

      {
        path: '/system',
        component: () => import('@/pages/system/SystemPage.vue'),
      },

    ],
  },
  {
    path: '/playing',
    component: () => import('@/pages/playing/PlayingFile.vue'),
  },
  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('@/pages/ErrorNotFound.vue'),
  },
];

export default routes;
