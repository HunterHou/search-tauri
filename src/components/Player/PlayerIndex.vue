<template>
    <div class="playDivMini">
        <vue3VideoPlay ref="vue3VideoPlayRef" style="max-height: 100vh" v-bind="optionsPC" @timeupdate="timeupdate($event)"
            @volumechange="volumechange" @ended="playNext(1)" />
        <div :style="{
            color: 'white',
            backgroundSize: '100% 100%',
            backgroundImage:
                'linear-gradient(to left, rgba(0,0,0,0.8), rgba(0,0,0,9)),url(\'' +
                getJpg(view.contextmenuTarget.Id) +
                '\')',
        }">
            <div class="my-header">
                <span style="color: bisque">{{ view.contextmenuTarget.Name }}</span>
                <div class="header-button">
                    <ElButton type="primary" @click="screenChange">{{
                        view.videoFullscreen ? "小屏" : "大屏"
                    }}</ElButton>
                    <ElButton type="primary" @click="hiddenPlayVideo">隐藏</ElButton>
                    <ElButton type="danger" @click="playNext(-1)">上一个</ElButton>
                    <ElButton type="danger" @click="playNext(1)">下一个</ElButton>
                    <ElButton type="primary" @click="closePlayVideo">关闭</ElButton>
                </div>
            </div>
            <div style="display: flex; flex-direction: row; justify-content: flex-start">
                <el-space spacer="|" wrap>
                    <ElInput style="margin-left: 1rem; min-width: 320px; width: auto" placeholder="请输入关键词"
                        v-model="view.playlistKeywords" clearable size="default" @change="(e) => queryRelation(e)">
                    </ElInput>
                    <ElButton type="primary" @click="moreTag = !moreTag">更多</ElButton>
                    <ElTag v-for="item in view.contextmenuTarget.Tags" key="default" type="danger" size="mini"
                        @click="queryRelation(item)">
                        {{ item }}
                    </ElTag>
                    <el-link :underline="false" type="success">{{ view.contextmenuTarget.Code }}
                    </el-link>
                    <el-link type="warning" size="large" style="margin-left: 0.5rem"
                        @click="queryRelation(view.contextmenuTarget.Actress)">
                        {{ view.contextmenuTarget.Actress }}
                    </el-link>

                    <el-link v-if="moreTag" type="warning" v-for="item in view.settingInfo.Tags" key="default" size="large"
                        :underline="false" style="margin-left: 0.5rem" @click="queryRelation(item)">
                        {{ item }}
                    </el-link>
                </el-space>
            </div>
            <div style="margin: 8px auto; height: 50vh; overflow: auto" v-if="view.playlist && view.playlist.length > 0">
                <ElSpace wrap size="default">
                    <ElCard v-for="play in view.playlist" :key="play" :body-style="{
                        padding: '2px',
                        color: view.contextmenuTarget.Id == play.Id ? 'green' : 'orange',
                        width: '156px',
                        minHeight: '80px',
                        backgroundSize: '100% 100%',
                        backgroundImage:
                            'linear-gradient(to left, rgba(205, 138, 50,0.1), rgba(205, 118, 50,0.2)),' +
                            'url(\'' +
                            getPng(play.Id) +
                            '\')',
                    }" @click="startPlayVideo(play)">
                        <span style="
                height: 3rem;
                margin-top: 60%;
                scale: 0.8;
                overflow: hidden;
                word-break: break-all;
                text-overflow: ellipsis;
                display: -webkit-box;
              ">{{ play.Name }}</span>
                    </ElCard>
                </ElSpace>
            </div>
        </div>
    </div>
</template>

<script setup>

import { reactive, ref } from 'vue'
import {
    volumechange,
} from "../../views/fileList/fileList";
import { MovieModel } from '../../views/fileList';
import { getFileStream, getJpg, getPng } from '../../utils/ImageUtils';
import { QueryFileList } from '../../api/file';
import { useSystemProperty } from "@/store/System";
import { ResultList } from '../../utils/ResultResponse';

const vue3VideoPlayRef = ref(null);
const isPlaying = ref(false);

const view = reactive({
    playlist: [],
    playlistKeywords: null,
    videoVisible: false,
    videoFullscreen: false,
    settingInfo: {},
    contextmenuTarget: {}
})
const queryParam = reactive({})
const currentTime = ref(1)
const moreTag = ref(false)
const systemProperty = useSystemProperty();

const hiddenPlayVideo = () => {
    view.videoVisible = false;
};

const timeupdate = (e) => {
    currentTime.value = e.target.currentTime;
}

const screenChange = () => {
    view.videoFullscreen = !view.videoFullscreen
    optionsPC.currentTime = currentTime.value
    setTimeout(() => {
        vue3VideoPlayRef.value?.play();
    }, 50);
}

const startPlayVideo = (item) => {
    if (!item) {
        return
    }
    view.videoVisible = true;
    view.contextmenuTarget = item;
    currentTime.value = 0;
    optionsPC.currentTime = 0;
    optionsPC.src = getFileStream(item.Id);
    queryRelation(item.Actress);
    isPlaying.value = true;
    setTimeout(() => {
        vue3VideoPlayRef.value?.play();
    }, 200);
};

defineExpose({
    startPlayVideo, hiddenPlayVideo
})


const closePlayVideo = () => {
    view.videoVisible = false;
    optionsPC.src = '';
    isPlaying.value = false;
};


const queryRelation = async (keywords) => {
    view.playlistKeywords = keywords
    const res = await QueryFileList({
        ...queryParam,
        Keyword: keywords,
        PageSize: 999,
        Page: 1,
    });
    const model = res;
    view.playlist = [...model.Data];
};

const playNext = (step) => {
    console.log("playNext");
    for (let i = 0; i < view.playlist.length; i++) {
        if (view.playlist[i].Id === view.contextmenuTarget.Id) {
            if (i == view.playlist.length - 1) {
                startPlayVideo(view.playlist[0], true);
                return;
            } else {
                if (i + step <= 0) {
                    startPlayVideo(view.playlist[0], true);
                    return;
                } else {
                    startPlayVideo(view.playlist[i + step], true);
                    return;
                }
            }
        }
    }
};

const optionsPC = reactive({
    width: "100%", //播放器高度
    height: "auto", //播放器高度
    color: "#409eff", //主题色
    title: "", //视频名称
    src: "", //视频源
    muted: false, //静音
    preload: "false",
    webFullScreen: false,
    speedRate: ["1.0", "1.25", "1.5", "2.0"], //播放倍速
    autoPlay: false, //自动播放
    loop: false, //循环播放
    mirror: false, //镜像画面
    ligthOff: true, //关灯模式
    currentTime: 10,
    volume: systemProperty.videoOptions.volume, //默认音量大小
    control: systemProperty.videoOptions.control, //是否显示控制
    controlBtns: systemProperty.videoOptions.controlBtns, //显示所有按钮,
});
</script>
