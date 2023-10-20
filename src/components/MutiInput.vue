<template>
  <div ref="target" style="width: 100%; height: 100%; padding: 1px">
    <span v-if="!editStyle" @click="editStyle = true">
      <q-chip
        dense
        color="orange"
        text-color="white"
        v-for="item in value"
        :key="item"
      >
        {{ item }}
      </q-chip>
    </span>
    <div v-if="editStyle">
      <q-chip
        dense
        color="green"
        text-color="white"
        v-for="item in value"
        :key="item"
        removable
        @remove="removeThis(item)"
      >
        {{ item }}
      </q-chip>
      <div style="display: flex; flex-direction: row">
        <q-input
          outlined
          v-model:model-value="inputText"
          class="inputText"
        ></q-input>
        <q-btn outlined @click="addValue">添加</q-btn>
      </div>
    </div>
  </div>
</template>
<script setup>
import { onMounted, ref, watch } from 'vue';
import { useMouseInElement } from '@vueuse/core';

const emits = defineEmits(['update:model-value', 'onchange']);

const target = ref(null);
const { isOutside } = useMouseInElement(target);

const value = ref([]);
const inputText = ref('');
const editStyle = ref(false);
const props = defineProps({
  modelValue: {
    type: Array,
    default: () => [],
  },
  options: {
    type: Array,
    default: () => [],
  },
});

watch(
  () => isOutside.value,
  (v) => {
    if (v) {
      setTimeout(() => {
        if (isOutside.value) {
          editStyle.value = false;
        } else {
          editStyle.value = true;
        }
      }, 3000);
    }
  }
);

const addValue = () => {
  const str = inputText.value;
  if (!value.value) {
    value.value = [];
  }
  if (value.value.indexOf(str) >= 0) {
    return;
  }
  value.value.push(str);
  inputText.value = null;
  emits('update:model-value', value.value);
  emits('onchange', value.value);
};

const removeThis = (str) => {
  if (!value.value) {
    value.value = [];
  }
  console.log(str);
  if (value.value.indexOf(str) < 0) {
    return;
  }
  value.value = value.value.filter((it) => it != str);
  emits('update:model-value', value.value);
  emits('onchange', value.value);
};

watch(
  () => props.modelValue,
  (e) => {
    value.value = e;
  }
);

onMounted(() => {
  value.value = props.modelValue;
});
</script>
<style lang="scss">
.checkItem {
  width: 8rem;
}

.inputText {
  width: 10rem;
}
</style>
