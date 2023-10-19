<script setup lang="ts">
import { ElMenu, ElMenuItem } from "element-plus";
import { computed, onMounted } from "vue";

import { useRouter } from "vue-router";
import { staticRoutes } from "../../route";
import { useSystemProperty } from "../../store/System";
import { ref } from "vue";
const { push } = useRouter();
const systemProperty = useSystemProperty();
const activeIndex = ref("1");
const logo = computed(() => {
  return systemProperty.Logo;
});
const handleSelect = (key: string, keyPath: string[]) => {
  push(key);
};
onMounted(() => {});
</script>
<template>
  <ElMenu
    background-color="#545c64"
    text-color="#fff"
    active-text-color="#ffd04b"
    :default-active="activeIndex"
    class="el-menu-demo"
    mode="horizontal"
    :ellipsis="false"
    @select="handleSelect"
    :router="true"
  >
    <ElMenuItem :index="logo.url">
      <ElImage v-if="logo.logo" :src="logo.logo"></ElImage>
      {{ logo.title }}
    </ElMenuItem>
    <div class="flex-grow" />
    <div v-for="item in staticRoutes">
      <ElMenuItem
        v-if="item.children && item.meta?.hidden == false"
        v-for="citem in item.children"
        :index="citem.path"
        >{{ citem.meta.title }}</ElMenuItem
      >
    </div>
  </ElMenu>
</template>
<style scoped>
.flex-grow {
  flex-grow: 1;
}

.el-header {
  height: 40px;
}

.el-menu-demo {
  height: 48px;
}

.el-menu-item {
  height: 48px;
}
</style>
