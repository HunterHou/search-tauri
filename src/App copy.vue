<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { GetSettingInfo } from "./api/setting";
import { useSystemProperty } from "./store/System";
import { useTitle } from '@vueuse/core'

const title = ref(useTitle())
const systemProperty = useSystemProperty()
const { currentRoute } = useRouter()

watch(currentRoute, () => {

  const routerName = currentRoute.value?.meta?.title as string
  // console.log('app', currentRoute.value)
  if (routerName) {
    title.value = routerName
  }
})
const key = computed(() => {
  // console.log('key', useRoute().path)
  return useRoute().path
})

const loadSetting = async () => {
  const res = await GetSettingInfo()
  if (res) {
    systemProperty.setSettingInfo(res)
  }

}
onMounted(() => {


  loadSetting()
})
</script>

<template>
  <section>
    <RouterView :key="key"></RouterView>
  </section>
</template>
<style>
</style>
