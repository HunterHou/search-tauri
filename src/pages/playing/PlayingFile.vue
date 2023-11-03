<script setup>
import { onMounted, ref } from 'vue'
import PlayingVideo from '@/components/PlayingVideo.vue';
import { useRoute } from 'vue-router';
import {
  FindFileInfo
} from '@/components/api/searchAPI';


const vue3VideoPlayRef = ref(null)
const { query } = useRoute();

const fetchSearch = async (id) => {
  const data = await FindFileInfo(id);
  console.log("playing",data)
  const { Name } = data
  document.title = Name
  vue3VideoPlayRef.value && vue3VideoPlayRef.value.open(data)
};
onMounted(() => {
  const { id } = query
  console.log(id)
  fetchSearch(id)
})
</script>
<template>
  <PlayingVideo ref="vue3VideoPlayRef" mode="page" />
</template>
