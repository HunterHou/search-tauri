<script setup lang="ts">
import MobileBarVue from '@/views/mobile/MobileBar.vue';
import { ElContainer } from "element-plus";
</script>

<template>
  <ElContainer>
    <div class="mobileMain">
      <RouterView></RouterView>
    </div>
    <MobileBarVue />
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
  }
  
  .mobileFooter {
    position: absolute;
  }
  </style>
