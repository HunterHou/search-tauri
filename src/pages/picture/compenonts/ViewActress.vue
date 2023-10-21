<template>
  <q-dialog ref="dialogRef" :title="view.item.Name" @hide="close" style="width: 80vw !important;"
    v-model:model-value="showDialog">
    <q-card class="q-dialog-plugin q-pa-md" style="width: 100%;max-width: 90vw !important" :title="view.item.Name">
      <h5>{{ view.item.Name }}【{{ view.item.SizeStr }}（{{ view.item.Cnt }}）】</h5>
      <q-img loading="lazy" fit="fit" v-for="item in view.prewiewImages" :key="item" :src="convertFileSrc(item)"
        style="width: 100%;height: auto;"></q-img>
    </q-card>
  </q-dialog>
</template>
<script setup>
import { useDialogPluginComponent } from 'quasar';
import { reactive, ref } from 'vue';

import { convertFileSrc } from '@tauri-apps/api/tauri';

const view = reactive({
  item: {},
  prewiewImages: []
})
const showDialog = ref(false)

const open = (data) => {
  showDialog.value = true
  view.item = data;
  view.prewiewImages = []
  setTimeout(() => {
    data.Images.forEach(element => {
      view.prewiewImages.push(element)
    });

  }, 100);
};

defineEmits([
  // REQUIRED; 需要明确指出
  // 组件通过 useDialogPluginComponent() 暴露哪些事件
  ...useDialogPluginComponent.emits,
]);


// onDialogOK, onDialogCancel
const { dialogRef, onDialogHide } = useDialogPluginComponent();

const close = () => {
  showDialog.value = false
}

defineExpose({
  open, close
});
</script>
