<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import {
  ElButton,
  ElMessage,
  ElTag,
  ElTable,
  ElTableColumn,
  ElBadge,
} from "element-plus";

import { RefreshIndex, OpenFolerByPath, DeleteFolerByPath } from "@/api/file";
import { TypeSizeMap, TagSizeMap, ScanTime } from "@/api/home";
import { onKeyStroke } from "@vueuse/core";
import { useSystemProperty } from "@/store/System";

const { push, go } = useRouter();
const systemProperty = useSystemProperty();
document.title = "首页";
const indexLoading = ref(false);
const view = reactive({
  tableData: [],
  indexLoading: false,
});
const tableData = ref<any>([]);
const tagData = ref<any>([]);
const scanTime = ref<any>([]);

onKeyStroke(["`"], (e) => {
  refreshIndex();
});

const folderGotoMenu = (Name) => {
  systemProperty.setPage(1);
  systemProperty.setKeyword(Name);
  systemProperty.setMovieType("");
  push("/filelist");
};

const gotoMenu = (data) => {
  const { IsDir, Name } = data;
  const movieType = !IsDir && Name !== "全部" ? Name : "";
  systemProperty.setPage(1);
  systemProperty.setKeyword(Name);
  systemProperty.setMovieType(movieType);
  push("/filelist");
};
const loadTypeSize = async () => {
  const res = await TypeSizeMap();
  if (res) {
    tableData.value = res;
    loadTagSize();
    loadScanTime();
  }
};

const loadTagSize = async () => {
  const res = await TagSizeMap();
  if (res) {
    tagData.value = (res as any).splice(0, 70);
  }
};
const loadScanTime = async () => {
  const res3 = await ScanTime();
  scanTime.value = res3;
};
onMounted(() => {
  loadTypeSize();
});

const openThis = async (index, data?) => {
  const { Name } = data;
  const res = await OpenFolerByPath({ dirpath: Name });
  if (res.Code == 200) {
    ElMessage.success("执行成功");
  } else {
    ElMessage.error("执行失败");
  }
};
const deleteThis = async (index, data?) => {
  const { Name } = data;
  const res = await DeleteFolerByPath({ dirpath: Name });
  if (res.Code == 200) {
    ElMessage.success("执行成功");
    refreshIndex();
  } else {
    ElMessage.error("执行失败");
  }
};
const refreshIndex = async () => {
  view.indexLoading = true;
  const res = await RefreshIndex();
  if (res.Code == 200) {
    view.indexLoading = false;
    ElMessage.success("执行成功");
    f5();
  } else {
    ElMessage.error("执行失败");
  }
};

const f5 = () => {
  setTimeout(() => {
    window.location.href = "/home";
  }, 200);
};
</script>

<template>
  <div>
    <h4 align="center">
      掃描结果分析
      <el-button
        type="primary"
        :loading="indexLoading"
        size="small"
        @click="refreshIndex()"
        >重建索引
      </el-button>
    </h4>
    <div v-if="tagData && tagData.length > 0">
      <el-link
        v-for="tag in tagData"
        :key="tag.Name"
        class="d-tag"
        :underline="false"
      >
        <el-tag size="default" :value="tag.Cnt" @click="gotoMenu(tag)">
          <el-badge :value="tag.Cnt" :max="999">
            <span style="font-size: 10px">
              <b
                >{{ tag.Name }} (<i>{{ tag.SizeStr }}</i
                >)
              </b>
            </span>
          </el-badge>
        </el-tag>
      </el-link>
    </div>
    <li class="left-div d-li" v-if="scanTime && scanTime.length > 0">
      <el-table :data="scanTime" align="center" :stripe="true">
        <el-table-column label="文件夹" header-align="left" align="left">
          <template #default="scope">
            <el-link
              :title="scope.row.Name"
              @click="folderGotoMenu(scope.row.Name)"
            >
              {{ scope.row.Name }}
            </el-link>
          </template>
        </el-table-column>
        <el-table-column label="数量" header-align="right" align="right">
          <template #default="scope">
            <el-link
              :title="scope.row.Size"
              @click="folderGotoMenu(scope.row.Size)"
            >
              {{ scope.row.Size }}
            </el-link>
          </template>
        </el-table-column>
        <el-table-column
          label="耗时"
          width="80px"
          header-align="right"
          align="right"
        >
          <template #default="scope">
            <el-link
              :title="scope.row.Cnt"
              @click="folderGotoMenu(scope.row.Name)"
            >
              {{ scope.row.Cnt }}&nbsp;ms
            </el-link>
          </template>
        </el-table-column>
      </el-table>
    </li>

    <li class="right-div d-li">
      <el-table
        :data="tableData"
        align="center"
        style="margin: 20px auto"
        :stripe="true"
        border
      >
        <el-table-column
          label="结果集"
          header-align="left"
          min-width="250px"
          align="left"
        >
          <template #default="scope">
            <el-link :title="scope.row.Name" @click="gotoMenu(scope.row)">
              {{ scope.row.Name }}
            </el-link>
          </template>
        </el-table-column>
        <el-table-column label="大小" header-align="right" align="right">
          <template #default="scope">
            <el-link :title="scope.row.Name" @click="gotoMenu(scope.row)">
              {{ scope.row.SizeStr }}
            </el-link>
          </template>
        </el-table-column>
        <el-table-column label="数量" header-align="right" align="right">
          <template #default="scope">
            <el-link :title="scope.row.Name" @click="gotoMenu(scope.row)">
              {{ scope.row.Cnt }}
            </el-link>
          </template>
        </el-table-column>
        <el-table-column
          prop="Name"
          label="操作"
          header-align="left"
          align="left"
        >
          <template #default="scope">
            <el-button
              v-if="!scope.row.IsDir"
              type="success"
              @click="gotoMenu(scope.row)"
              >前往
            </el-button>
            <el-button
              size="small"
              v-if="scope.row.IsDir"
              type="info"
              @click="openThis(scope.$index, scope.row)"
              >打开
            </el-button>
            <el-button
              size="small"
              v-if="scope.row.IsDir"
              type="danger"
              @click="deleteThis(scope.$index, scope.row)"
              >删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </li>
  </div>
</template>

<style>
.d-li {
  overflow: inherit;
  display: block;
  float: left;
}

.d-tag {
  margin: 4px 8px;
}

.left-div {
  min-width: 250px;
  width: 38%;
}

.right-div {
  margin: 0 10px;
  width: 60%;
  min-width: 350px;
}

.e-tag {
  margin-right: 14px;
  margin-top: 2px;
}
</style>
