<template>
  <div class="q-pa-md">
    <div class="row justify-center q-gutter-sm">
      <q-btn :loading="refreshIndexLoading" color="red" @click="refreshIndex">
        扫描
        <template v-slot:loading> 执行中 </template>
      </q-btn>
      <q-btn-toggle
        v-model="view.queryParam.SortField"
        @update:model-value="fetchSearch"
        toggle-color="primary"
        :options="[
          { label: '时', value: 'MTime' },
          { label: '容', value: 'Size' },
          { label: '名', value: 'Code' },
        ]"
      />
      <q-btn-toggle
        v-model="view.queryParam.SortType"
        @update:model-value="fetchSearch"
        toggle-color="primary"
        :options="[
          { label: '正', value: 'asc' },
          { label: '倒', value: 'desc' },
        ]"
      />

      <q-btn-toggle
        v-model="view.queryParam.MovieType"
        @update:model-value="fetchSearch"
        toggle-color="primary"
        :options="[
          { label: '全部', value: '' },
          { label: '骑兵', value: '骑兵' },
          { label: '步兵', value: '步兵' },
          { label: '国产', value: '国产' },
          { label: '斯巴达', value: '斯巴达' },
          { label: '漫动', value: '漫动' },
          { label: '无', value: '无' },
        ]"
      />
      <q-input
        v-model="view.queryParam.Keyword"
        :dense="true"
        filled
        clearable
        @update:model-value="fetchSearch"
      />
      <q-checkbox
        v-model="view.queryParam.OnlyRepeat"
        @update:model-value="fetchSearch"
        label="重"
      />
    </div>
    <q-page-sticky
      position="bottom"
      style="z-index: 9; background-color: rgba(0, 0, 0, 0.6)"
    >
      <div class="q-pa-lg flex flex-center">
        <q-pagination
          v-model="view.queryParam.Page"
          @update:model-value="currentPageChange"
          color="purple"
          :ellipses="false"
          :max="view.resultData.TotalPage || 0"
          :max-pages="6"
          boundary-numbers
        />
      </div>
    </q-page-sticky>

    <div class="row justify-center q-gutter-sm">
      <q-card
        class="q-ma-sm example-item"
        v-for="item in view.resultData.Data"
        :key="item.Id"
      >
        <q-img
          fit="cover"
          easier
          draggable
          :src="getPng(item.Id)"
          class="item-img"
          @click="openDialog(item)"
        >
          <div
            style="
              padding: 0;
              margin: 0;
              background-color: rgba(0, 0, 0, 0);
              display: flex;
              flex-direction: row;
              justify-content: space-between;
              width: 100%;
            "
          >
            <div
              @click.stop="() => {}"
              style="
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                width: fit-content;
              "
            >
              <q-chip
                square
                color="red"
                text-color="white"
                v-for="tag in item.Tags"
                :key="tag"
                style="margin-left: 0px; padding: 0 4px"
              >
                <span
                  @click="
                    view.queryParam.Keyword = tag;
                    fetchSearch();
                  "
                  >{{ tag }}</span
                >
              </q-chip>
            </div>
            <q-chip
              @click.stop="() => {}"
              square
              color="green"
              text-color="white"
              style="width: fit-content; margin-right: 0px; padding: 0 6px"
            >
              <span
                @click="
                  view.queryParam.Keyword = item.MovieType;
                  fetchSearch();
                "
              >
                {{ item.MovieType }}</span
              >
            </q-chip>
          </div>
          <div
            class="absolute-bottom text-body1 text-center"
            style="padding: 4px"
            @click.stop="() => {}"
          >
            <q-btn
              flat
              style="color: #59d89d"
              :label="item.Actress?.substring(0, 10)"
              @click="
                view.queryParam.Keyword = item.Actress;
                fetchSearch();
              "
            />
            <q-btn
              flat
              style="color: goldenrod"
              :label="item.Code?.substring(0, 10)"
              @click="copy(item.Code)"
            />
          </div>
        </q-img>
        <q-card-section>
          <div class="text-subtitle2">
            <q-chip
              @click.stop="() => {}"
              square
              color="green"
              text-color="white"
              style="padding: 0px 4px"
            >
              {{ item.SizeStr }} </q-chip
            >{{ item.Title }}
          </div>
        </q-card-section>
      </q-card>
    </div>
  </div>
</template>

<script setup>
import { useQuasar } from 'quasar';
import { ref } from 'vue';

import { onMounted, reactive } from 'vue';
import { useRoute } from 'vue-router';
import { getPng } from '../../components/utils/images';
import { SearchAPI, RefreshAPI } from '../../components/api/searchAPI';
import { useSystemProperty } from '../../stores/System';
import { useElementSize } from '@vueuse/core';

import { useClipboard } from '@vueuse/core';

const source = ref('Hello');
const { copy } = useClipboard({ source });

const el = (ref < HTMLElement) | (null > null);
useElementSize(el);

const systemProperty = useSystemProperty();

const $q = useQuasar();
const view = reactive({
  currentData: {},
  queryParam: {
    Keyword: '',
    MovieType: '',
    OnlyRepeat: false,
    Page: 1,
    PageSize: 10,
    SortField: 'MTime',
    SortType: 'desc',
  },
  resultData: {},
  fullscreen: false,
});

const openDialog = (item) => {
  view.currentData = item;
  systemProperty.Playing = item;
  systemProperty.drawerRight = true;
};

const currentPageChange = (e) => {
  console.log('view.queryParam.Page', e);
  fetchSearch();
};

const fetchSearch = async () => {
  systemProperty.syncSearchParam(view.queryParam);
  localStorage.setItem('queryParam', JSON.stringify(view.queryParam));
  const { data } = await SearchAPI('/api/movieList', view.queryParam);
  console.log(data);
  view.resultData = data;
};

const refreshIndexLoading = ref(false);
const refreshIndex = async () => {
  refreshIndexLoading.value = true;
  const { Code, Message } = await RefreshAPI();
  if (Code === '200') {
    $q.notify(Message);
  }
  refreshIndexLoading.value = false;
};
const thisRoute = useRoute();

onMounted(() => {
  const { Page, PageSize, MovieType, SortField, SortType, Keyword, showStyle } =
    thisRoute.query;
  if (Page && PageSize) {
    view.queryParam.Page = Number(Page);
    view.queryParam.PageSize = Number(PageSize);
    view.queryParam.MovieType = MovieType;
    view.queryParam.SortField = SortField;
    view.queryParam.SortType = SortType;
    view.queryParam.Keyword = Keyword;
    view.queryParam.Keyword = Keyword;
    view.queryParam.showStyle = showStyle;
  } else {
    const storage = JSON.parse(localStorage.getItem('queryParam'));
    if (storage) {
      view.queryParam = storage;
    }
  }
  fetchSearch();
});
</script>
<style lang="scss" scoped>
.example-item {
  height: auto;
  width: 240px;
  max-height: 500px;
}

.item-img {
  max-height: 300px;
}
</style>
