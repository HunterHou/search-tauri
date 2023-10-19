<script setup lang="ts">
import { ElMenu, ElMenuItem } from "element-plus";
import {  onMounted } from "vue";

import { useRouter } from "vue-router";
import { staticRoutes } from "../../route";
import { ref } from "vue";
const { push } = useRouter();
const activeIndex = ref("1");
const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
  push(key);
};
onMounted(() => { });
</script>
<template>
  <ElMenu background-color="#545c64" text-color="#fff" active-text-color="#ffd04b" :default-active="activeIndex"
    class="el-menu-demo" mode="horizontal" :ellipsis="false" @select="handleSelect" :router="true">
    <div v-for="item in staticRoutes">
      <ElMenuItem v-if="item.children && item.meta?.hidden == false" v-for="citem in item.children" :index="citem.path">{{
        citem?.meta.title }}</ElMenuItem>
    </div>
  </ElMenu>
</template>
<style scoped>
.flex-grow {
  flex-grow: 1;
}

.el-menu-demo{
  margin: 1px;
  padding: 0;
  height: 48px;
}

</style>
