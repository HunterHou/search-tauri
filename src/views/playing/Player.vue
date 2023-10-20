<template>
    <Player ref="player" />
</template>
<script setup lang="ts">

import { ref } from 'vue';
import { useRoute } from 'vue-router'
import Player from '@/components/Player/PlayerIndex.vue'

import { FindFileInfo } from '../../api/file';

const { params } = useRoute()
const player = ref(null)
const loadingMovie = async (id:string) => {
    const res = await FindFileInfo(id)
    const { Name } = res
    player.value.startPlayVideo(res)
    document.title = Name
}
loadingMovie(params.id[0])
</script>