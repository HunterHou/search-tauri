<template>
  <div class="q-pa-md">
    <q-page-sticky position="top" :offset="[20, 20]" style="width: 100%">
      <q-tabs v-model="tab" style="width: 100%" dense no-caps inline-label class="shadow-2" active-color="primary"
        indicator-color="primary" align="justify">
        <q-tab name="search" label="搜索设置" />
        <q-tab name="base" label="基础设置" />
        <q-tab name="system" label="系统设置" />
      </q-tabs>
    </q-page-sticky>
    <div style="margin: 40px 10px 40px 10px">
      <div style="
          display: flex;
          flex-direction: row;
          justify-content: space-between;
          width: 100%;
        ">
        <a :href="view.ipAddr">访问： {{ view.ipAddr }}</a>
      </div>
      <q-separator />
      <q-tab-panels v-model="tab" animated>
        <q-tab-panel name="search">
          <q-field color="purple-12" label="Buttons" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <!-- <MutiInput
                v-model="view.settingInfo.Buttons"
                @onchange="(arr) => (view.settingInfo.Buttons = arr)"
              /> -->
              <MutiSelector v-bind:model-value="view.settingInfo.Buttons" :options="buttonEnum"
                @onchange="(arr) => (view.settingInfo.Buttons = arr)" />
            </template>
          </q-field>

          <q-field color="purple-12" label="Dirs" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <MutiSelector v-bind:model-value="view.settingInfo.Dirs" :options="view.settingInfo.DirsLib"
                @onchange="(arr) => (view.settingInfo.Dirs = arr)" />
            </template>
          </q-field>
          <q-field color="purple-12" label="MovieTypes" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <MutiInput v-model="view.settingInfo.MovieTypes" @onchange="(arr) => (view.settingInfo.MovieTypes = arr)" />
            </template>
          </q-field>
          <q-field color="purple-12" label="VideoTypes" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <MutiSelector v-bind:model-value="view.settingInfo.VideoTypes" :options="view.settingInfo.Types"
                @onchange="(arr) => (view.settingInfo.VideoTypes = arr)" />
            </template>
          </q-field>
          <q-field color="purple-12" label="Tags" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <MutiSelector v-bind:model-value="view.settingInfo.Tags" :options="view.settingInfo.TagsLib"
                @onchange="(arr) => (view.settingInfo.Tags = arr)" />
            </template>
          </q-field>
        </q-tab-panel>

        <q-tab-panel name="base">
          <q-input v-model="view.settingInfo.ControllerHost" label="ControllerHost" />
          <q-input v-model="view.settingInfo.ImageHost" label="ImageHost" />
          <q-input v-model="view.settingInfo.StreamHost" label="StreamHost" />
          <q-input v-model="view.settingInfo.BaseUrl" label="BaseUrl" />
          <q-input v-model="view.settingInfo.OMUrl" label="OMUrl" />

          <q-field color="purple-12" label="DirsLib" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <MutiInput v-model="view.settingInfo.DirsLib" @onchange="(arr) => (view.settingInfo.DirsLib = arr)" />
            </template>
          </q-field>
          <q-field color="purple-12" label="TagsLib" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <MutiInput v-model="view.settingInfo.TagsLib" @onchange="(arr) => (view.settingInfo.TagsLib = arr)" />
            </template>
          </q-field>
          <q-field color="purple-12" label="Types" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <MutiInput v-model="view.settingInfo.Types" @onchange="(arr) => (view.settingInfo.Types = arr)" />
            </template>
          </q-field>
        </q-tab-panel>

        <q-tab-panel name="system">
          <!-- <q-editor v-model="view.settingInfo.SystemHtml" min-height="5rem" /> -->
          <q-editor v-model="view.settingInfo.SystemHtml" :dense="$q.screen.lt.md" :toolbar="[
            [
              {
                label: $q.lang.editor.align,
                icon: $q.iconSet.editor.align,
                fixedLabel: true,
                list: 'only-icons',
                options: ['left', 'center', 'right', 'justify']
              },
              {
                label: $q.lang.editor.align,
                icon: $q.iconSet.editor.align,
                fixedLabel: true,
                options: ['left', 'center', 'right', 'justify']
              }
            ],
            [
              'bold',
              'italic',
              'strike',
              'underline',
              'subscript',
              'superscript'
            ],
            ['token', 'hr', 'link', 'custom_btn'],
            ['print', 'fullscreen'],
            [
              {
                label: $q.lang.editor.formatting,
                icon: $q.iconSet.editor.formatting,
                list: 'no-icons',
                options: ['p', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'code']
              },
              {
                label: $q.lang.editor.fontSize,
                icon: $q.iconSet.editor.fontSize,
                fixedLabel: true,
                fixedIcon: true,
                list: 'no-icons',
                options: [
                  'size-1',
                  'size-2',
                  'size-3',
                  'size-4',
                  'size-5',
                  'size-6',
                  'size-7'
                ]
              },
              {
                label: $q.lang.editor.defaultFont,
                icon: $q.iconSet.editor.font,
                fixedIcon: true,
                list: 'no-icons',
                options: [
                  'default_font',
                  'arial',
                  'arial_black',
                  'comic_sans',
                  'courier_new',
                  'impact',
                  'lucida_grande',
                  'times_new_roman',
                  'verdana'
                ]
              },
              'removeFormat'
            ],
            ['quote', 'unordered', 'ordered', 'outdent', 'indent'],

            ['undo', 'redo'],
            ['viewsource']
          ]" :fonts="{
  arial: 'Arial',
  arial_black: 'Arial Black',
  comic_sans: 'Comic Sans MS',
  courier_new: 'Courier New',
  impact: 'Impact',
  lucida_grande: 'Lucida Grande',
  times_new_roman: 'Times New Roman',
  verdana: 'Verdana'
}" />
        </q-tab-panel>
      </q-tab-panels>
    </div>
    <q-page-sticky position="bottom" :offset="[20, 20]">
      <q-btn color="blue" style="width: 10rem" @click="submitForm">提交</q-btn>
    </q-page-sticky>
  </div>
</template>

<script setup>
import { useQuasar } from 'quasar';

import { onMounted, reactive, ref } from 'vue';
import {
  GetSettingInfo,
  PostSettingInfo,
  GetIpAddr
} from '../../components/api/settingAPI';
import MutiSelector from '../../components/MutiSelector.vue';
import MutiInput from '../../components/MutiInput.vue';
import { buttonEnum } from '../../components/model/Setting';

const $q = useQuasar();
const tab = ref('search');
const view = reactive({
  settingInfo: {},
  ipAddr: ''
});

const submitForm = async () => {
  const { Code, Message } = await PostSettingInfo(view.settingInfo);
  console.log(Code, Message);
  if (Code != 200) {
    $q.notify({ type: 'positive', message: `${Message}`, multiLine: true, position: 'bottom-right' });
    // window.location.reload()
  } else {
    $q.notify({ type: 'negative', message: `${Message}`, multiLine: true, position: 'bottom-right' });
  }
};

const fetchSearch = async () => {
  const { data } = await GetSettingInfo();
  console.log(data);
  view.settingInfo = data;
};

const queryIpAddr = async () => {
  const { Code, Data } = await GetIpAddr();
  if (Code == '200') {
    view.ipAddr = `http://${Data}:10081`;
  }
};

onMounted(() => {
  document.title = '设置'
  fetchSearch();
  queryIpAddr();
});
</script>
<style lang="scss" scoped></style>
