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
      <q-checkbox
        class="checkItem"
        v-model="value"
        v-for="item in props.options"
        :key="item"
        :val="item"
        :label="item"
        color="teal"
        @update:model-value="updateValue"
        cl
      />
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

const updateValue = (arr) => {
  emits('update:model-value', arr);
  emits('onchange', arr);
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
</style>
