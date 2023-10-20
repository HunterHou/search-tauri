<template>
  <q-item clickable tag="a" target="_self" :to="link">
    <!-- <q-item-section
      avatar
    >
    </q-item-section> -->
    <q-item-section>
      <q-item-label
        ><q-icon v-if="icon" :name="icon" />{{ title }}</q-item-label
      >
      <!-- <q-item-label caption>{{ caption }}</q-item-label> -->
    </q-item-section>
  </q-item>
</template>

<script setup lang="ts">
export interface EssentialLinkProps {
  title: string;
  caption?: string;
  link?: string;
  icon?: string;
}
withDefaults(defineProps<EssentialLinkProps>(), {
  caption: '',
  link: '#',
  icon: '',
});
</script>
