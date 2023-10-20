<template>
  <div class="q-pa-md">
    <q-page-sticky style="z-index: 9;" position="left" :offset="[0, 0]">
      <q-btn round class="page-sticky" color="amber" text-color="black" icon="keyboard_arrow_left"
        v-if="view.queryParam.Page > 1" @click="nextPage(-1)"></q-btn>
    </q-page-sticky>

    <q-page-sticky style="z-index: 9;" position="right" :offset="[10, 10]">
      <q-btn round class="page-sticky" color="secondary" text-color="black" icon="keyboard_arrow_right"
        @click="nextPage(1)"></q-btn>
    </q-page-sticky>
    <div class="row justify-center q-gutter-sm">
      <q-btn-toggle v-model="view.queryParam.SortField" @update:model-value="fetchSearch" toggle-color="primary" :options="[
        { label: '时', value: 'MTime' },
        { label: '容', value: 'Size' },
        { label: '名', value: 'Code' },
      ]" />
      <q-btn-toggle v-model="view.queryParam.SortType" @update:model-value="fetchSearch" toggle-color="primary" :options="[
        { label: '正', value: 'asc' },
        { label: '倒', value: 'desc' },
      ]" />
      <q-input v-model="view.queryParam.Keyword" :dense="true" filled clearable @update:model-value="fetchSearch" />
      <q-pagination v-model="view.queryParam.Page" @update:model-value="currentPageChange" color="purple"
        :ellipses="false" :max="view.resultData.TotalCnt || 0" :max-pages="6" boundary-numbers />
    </div>
    <div style="display: flex; flex-direction: row; flex-wrap: wrap">
      <q-card class="q-ma-sm example-item" v-for="item in view.resultData.Data" :key="item.Id">
        <q-img fit="fill" :src="getActressImage(item.Name)" class="item-img" @click="searchFiles(item.Name)">
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
              <q-chip square color="red" text-color="white" style="margin-left: 0px; padding: 0 4px">
                <span>{{ item.SizeStr }}</span>
              </q-chip>
            </div>
            <q-chip @click.stop="() => { }" square color="green" text-color="white"
              style="width: fit-content; margin-right: 0px; padding: 0 6px">
              <span> {{ item.Cnt }}</span>
            </q-chip>
          </div>
          <div class="absolute-bottom text-body1 text-center" style="padding: 4px" @click.stop="() => { }">
            <q-btn flat style="color: #59d89d" :label="item.Name?.substring(0, 10)" />
          </div>
        </q-img>
      </q-card>
    </div>
  </div>
</template>

<script setup>
import { onMounted, reactive } from 'vue';
import { getActressImage } from '../../components/utils/images';
import { QueryActressList } from '../../components/api/actressAPI';
import { useSystemProperty } from '../../stores/System';
import { useRouter } from 'vue-router';

const { push } = useRouter();

const systemProperty = useSystemProperty();

const view = reactive({
  currentData: {},
  queryParam: {
    Keyword: '',
    MovieType: '',
    OnlyRepeat: false,
    Page: 1,
    PageSize: 50,
    SortField: 'MTime',
    SortType: 'desc',
  },
  resultData: {},
  fullscreen: false,
});

const searchFiles = (name) => {
  systemProperty.FileSearchParam.Keyword = name;
  console.log(name);
  push({ path: '/search', query: { Keyword: name, from: 'index' } });
};

const currentPageChange = (e) => {
  console.log('view.queryParam.Page', e);
  fetchSearch();
};

const nextPage = (n) => {
  view.queryParam.Page = view.queryParam.Page + n
  currentPageChange()
}

const fetchSearch = async () => {
  const { data } = await QueryActressList(view.queryParam);
  console.log(data);
  view.resultData = data;
};

onMounted(() => {
  document.title = '图鉴'
  fetchSearch();
});
</script>
<style lang="scss" scoped>
.example-item {
  width: fit-content;
  height: fit-content;
}

.item-img {
  width: 220px;
  height: 282px;
}

.page-sticky {
  width: 4rem;
  height: 3rem;
}
</style>
