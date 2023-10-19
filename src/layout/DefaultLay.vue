<script setup lang="ts">
import { ElContainer, ElHeader } from "element-plus";
import HeaderVue from "../components/Header/Header.vue";


</script>

<template>
    <ElContainer>
        <ElHeader >
            <HeaderVue />
        </ElHeader>
        <div class="main">
            <RouterView></RouterView>
        </div>
        <ElFooter >
            <hr />
        </ElFooter>
    </ElContainer>
</template>
<style>
html {
  font-family: "Source Sans Pro", -apple-system, BlinkMacSystemFont, "Segoe UI",
    Roboto, "Helvetica Neue", Arial, sans-serif;
  word-spacing: 1px;
  -ms-text-size-adjust: 100%;
  -webkit-text-size-adjust: 100%;
  -moz-osx-font-smoothing: grayscale;
  -webkit-font-smoothing: antialiased;
  box-sizing: border-box;
  --el-header-height: 48px;
}

.main {
  padding-right: 20px;
  padding-bottom: 25px;
  padding-left: 20px;
}
</style>