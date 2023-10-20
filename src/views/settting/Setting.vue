<template>
  <div align="center" style="margin-top: -30px">
    <ElForm
        label-width="160px"
        ref="form"
        :model="view.form"
        label-position="right"
    >
      <h3>
        <ElButton
            style="float: left"
            size="default"
            type="danger"
            @click="shutDownHandler"
        >关闭计算机
        </ElButton
        >
        <a :href="'http://' + view.ipAddr + ':10081'">{{
            view.ipAddr + ":10081"
          }}</a>
        <span style="float: right">{{ formatted }} </span>
      </h3>
      <div class="container">
        <ElTabs v-model="activeName" type="card" @tab-click="handleClick">
          <ElTabPane label="扫描设置" name="first">
            <ElSwitch
                v-model="view.form.IsDb"
                size="default"
                active-text="数据库"
                inactive-text="算法"
            />
            <ElFormItem label="图片类型">
              <ElSelect
                  v-model="view.form.ImageTypes"
                  multiple
                  placeholder="请选择"
                  style="width: 90%"
                  size="default"
              >
                <ElOption
                    v-for="item in view.form.Types"
                    :key="item"
                    :label="item"
                    :value="item"
                >
                </ElOption>
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="文档类型">
              <ElSelect
                  v-model="view.form.DocsTypes"
                  multiple
                  placeholder="请选择"
                  style="width: 90%"
                  size="default"
              >
                <ElOption
                    v-for="item in view.form.Types"
                    :key="item"
                    :label="item"
                    :value="item"
                >
                </ElOption>
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="视频类型">
              <ElSelect
                  v-model="view.form.VideoTypes"
                  multiple
                  placeholder="请选择"
                  style="width: 90%"
                  size="default"
              >
                <ElOption
                    v-for="item in view.form.Types"
                    :key="item"
                    :label="item"
                    :value="item"
                >
                </ElOption>
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="定制按钮">
              <el-checkbox-group
                  v-model="view.form.Buttons"
                  size="large"
              >
                <el-checkbox
                    v-for="item in buttonEnum"
                    :key="item"
                    :label="item"
                >
                  {{ item }}
                </el-checkbox>
              </el-checkbox-group>
            </ElFormItem>
            <ElFormItem label="热门标签">
              <div style="width: 90%">
                <ElTag
                    v-for="iteTag in view.form.Tags"
                    size="default"
                    :key="iteTag"
                    closable
                    style="margin-right: 5px"
                    @close="removeTag(iteTag)"
                >
                  {{ iteTag }}
                </ElTag>

                <ElPopover placement="left" width="400px;" trigger="click">
                  <template #reference>
                    <ElLink
                        type="success"
                        icon="el-icon-edit"
                        @click="
                        () => {
                          makeTabLibData();
                          view.popTagLibData = view.form.tagLibData;
                        }
                      "
                    >添加
                    </ElLink>
                  </template>
                  <template #default>
                    <ElTransfer
                        :titles="['标签库', '已选']"
                        v-model="view.form.Tags"
                        target-order="push"
                        :data="view.popTagLibData"
                    >
                    </ElTransfer>
                  </template>
                </ElPopover>
              </div>
            </ElFormItem>

            <ElFormItem label="已选路徑">
              <ElCheckbox
                  size="default"
                  :indeterminate="view.isIndeterminateDir"
                  v-model="checkAll"
                  @change="handleCheckAllChange"
              >全选
              </ElCheckbox>
              <ElCheckboxGroup
                  v-model="view.form.Dirs"
                  @change="handleCheckedCitiesChange"
              >
                <ElCheckbox
                    size="default"
                    v-for="dir in view.form.DirsLib"
                    :label="dir"
                    :key="dir"
                >{{ dir }}
                </ElCheckbox>
              </ElCheckboxGroup>
            </ElFormItem>
            <ElFormItem label="备注">
              <ElInput
                  type="textarea"
                  :rows="8"
                  v-model="view.form.Remark"
                  style="width: 90%; margin-bottom: 20px"
              >
              </ElInput>
            </ElFormItem>
          </ElTabPane>

          <ElTabPane label="基础配置" name="second">
            <ElFormItem label="请求服务器">
              <ElInput
                  v-model="view.form.ControllerHost"
                  style="width: 90%"
                  size="default"
              ></ElInput>
            </ElFormItem>
            <ElFormItem label="图片服务器">
              <ElInput
                  v-model="view.form.ImageHost"
                  style="width: 90%"
                  size="default"
              ></ElInput>
            </ElFormItem>
            <ElFormItem label="流服务器">
              <ElInput
                  v-model="view.form.StreamHost"
                  style="width: 90%"
                  size="default"
              ></ElInput>
            </ElFormItem>
            <ElFormItem label="URL">
              <ElInput
                  v-model="view.form.BaseUrl"
                  style="width: 90%"
                  size="default"
              ></ElInput>
            </ElFormItem>
            <ElFormItem label="OM-URL">
              <ElInput
                  v-model="view.form.OMUrl"
                  style="width: 90%"
                  size="default"
              ></ElInput>
            </ElFormItem>
            <ElFormItem label="枚举文件类型">
              <el-tag
                  v-for="tag in view.form.Types"
                  :key="tag"
                  class="inputTag"
                  closable
                  :disable-transitions="false"
                  @close="handleClose(tag, 'Types')"
              >
                {{ tag }}
              </el-tag>
              <el-input
                  v-if="view.typeVisible"
                  ref="InputRef"
                  v-model="view.inputValue"
                  class="ml-1 w-20"
                  size="small"
                  @keyup.enter="enterInput('typeVisible', 'Types')"
                  @blur="enterInput('typeVisible', 'Types')"
              />
              <el-button
                  v-else
                  class="button-new-tag ml-1"
                  size="small"
                  @click="showInput('typeVisible')"
              >
                + New
              </el-button>
            </ElFormItem>
            <ElFormItem label="视频类型">
              <el-tag
                  v-for="tag in view.form.MovieTypes"
                  :key="tag"
                  class="inputTag"
                  closable
                  :disable-transitions="false"
                  @close="handleClose(tag, 'MovieTypes')"
              >
                {{ tag }}
              </el-tag>
              <el-input
                  v-if="view.MovieTypesVisible"
                  ref="InputRef"
                  v-model="view.inputValue"
                  class="ml-1 w-20"
                  size="small"
                  @keyup.enter="enterInput('MovieTypesVisible', 'MovieTypes')"
                  @blur="enterInput('MovieTypesVisible', 'MovieTypes')"
              />
              <el-button
                  v-else
                  class="button-new-tag ml-1"
                  size="small"
                  @click="showInput('MovieTypesVisible')"
              >
                + New
              </el-button>
            </ElFormItem>
            <ElFormItem label="标签库">
              <el-tag
                  v-for="tag in view.form.TagsLib"
                  :key="tag"
                  class="inputTag"
                  closable
                  :disable-transitions="false"
                  @close="handleClose(tag, 'TagsLib')"
              >
                {{ tag }}
              </el-tag>
              <el-input
                  v-if="view.TagsLibVisible"
                  ref="InputRef"
                  v-model="view.inputValue"
                  class="ml-1 w-20"
                  size="small"
                  @keyup.enter="enterInput('TagsLibVisible', 'TagsLib')"
                  @blur="enterInput('TagsLibVisible', 'TagsLib')"
              />
              <el-button
                  v-else
                  class="button-new-tag ml-1"
                  size="small"
                  @click="showInput('TagsLibVisible')"
              >
                + New
              </el-button>
            </ElFormItem>
            <ElFormItem label="路徑库">
              <el-tag
                  v-for="tag in view.form.DirsLib"
                  :key="tag"
                  class="inputTag"
                  closable
                  :disable-transitions="false"
                  @close="handleClose(tag, 'DirsLib')"
              >
                {{ tag }}
              </el-tag>
              <el-input
                  v-if="view.DirsLibVisible"
                  ref="InputRef"
                  v-model="view.inputValue"
                  class="ml-1 w-20"
                  size="small"
                  @keyup.enter="enterInput('DirsLibVisible', 'DirsLib')"
                  @blur="enterInput('DirsLibVisible', 'DirsLib')"
              />
              <el-button
                  v-else
                  class="button-new-tag ml-1"
                  size="small"
                  @click="showInput('DirsLibVisible')"
              >
                + New
              </el-button>
            </ElFormItem>
            <ElFormItem label="备注">
              <ElInput
                  type="textarea"
                  :rows="8"
                  v-model="view.form.Remark"
                  style="width: 90%; margin-bottom: 20px"
              >
              </ElInput>
            </ElFormItem>
          </ElTabPane>

          <ElTabPane label="系统信息" name="third">
            <div style="text-align: left;">
              <v-md-editor
                  v-model="view.form.SystemHtml"
                  height="700px"
              ></v-md-editor>
              <!-- <ElInput type="textarea" :rows="8" v-model="view.form.SystemHtml" /> -->
            </div>

          </ElTabPane>
        </ElTabs>
      </div>

      <div>
        <ElButton
            class="return"
            type="warning"
            align-text="center"
            @click="goMenu"
        >返回
        </ElButton
        >
      </div>
      <div>
        <ElButton
            class="submit"
            type="primary"
            align-text="center"
            @click="submitForm"
        >提交
        </ElButton
        >
      </div>
    </ElForm>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, reactive, ref} from "vue";
import {useRouter} from "vue-router";
import {
  ElButton,
  ElCheckbox,
  ElCheckboxGroup,
  ElForm,
  ElFormItem,
  ElInput,
  ElLink,
  ElMessage,
  ElOption,
  ElPopover,
  ElSelect,
  ElTabPane,
  ElTabs,
  ElTag,
  ElTransfer,
} from "element-plus";
import {GetIpAddr, GetSettingInfo, GetShutDown, PostSettingInfo,} from "@/api/setting";
import {useDateFormat, useNow} from "@vueuse/core";
import {buttonEnum, SettingInfo} from "@/views/settting/index";

const formatted = useDateFormat(useNow(), "YYYY-MM-DD HH:mm:ss");

const {go} = useRouter();


const view = reactive({
  form: new SettingInfo(),
  ipAddr: "",
  inputValue: "",
  typeVisible: false,
  MovieTypesVisible: false,
  TagsLibVisible: false,
  DirsLibVisible: false,
  isIndeterminateDir: false,
  popTagLibData: [],
  isIndeterminate: false,
  loading: false,
});
const activeName = ref("second");

const showInput = (visible) => {
  view[visible] = true;
  console.log(visible);
  visible = true;
};
const enterInput = (visible, arr) => {
  if (!view.inputValue) {
    return;
  }
  if(!view.form[arr]){
    view.form[arr] = []
  }
  if (view.form[arr].indexOf(view.inputValue) < 0) {
    view.form[arr].push(view.inputValue);
  }
  view[visible] = false;
  view.inputValue = null;
};

const handleClose = (tag, arr) => {
  view.form[arr].splice(view.form[arr].indexOf(tag), 1);
};

const checkAll = computed(() => {
  return view.form?.Dirs?.length === view.form?.DirsLib?.length;
});
const removeTag = (val) => {
  const idx = view.form.Tags.indexOf(val);
  view.form.Tags.splice(idx, 1);
};
const goMenu = () => {
  go(-1);
};

const handleClick = () => {
  // console.log('val')
};
const handleCheckAllChange = (val) => {
  view.form.Dirs = val ? view.form.DirsLib : [];
  view.isIndeterminate = false;
  // if (view.form?.Dirs?.length === view.form?.DirsLib?.length) {
  //   checkAll.value = true
  // } else {
  //   checkAll.value = false
  // }
};
const handleCheckedCitiesChange = () => {
  console.log(view.form?.Dirs.length, view.form?.Dirs);
  console.log(view.form?.DirsLib.length, view.form?.DirsLib);
  // if (view.form?.Dirs?.length === view.form?.DirsLib?.length) {
  //   checkAll.value = true
  // } else {
  //   checkAll.value = false
  // }
};

const submitForm = async () => {
  const postForm = {...view.form, BaseDir: view.form.Dirs};
  view.loading = true;
  // console.log(postForm);
  const res = await PostSettingInfo(postForm);
  if (res) {
    go(0);
  }
};

const makeTabLibData = () => {
  const dataLib = [];
  const {TagsLib = []} = view.form;
  if (TagsLib) {
    for (var i = 0; i < TagsLib.length; i++) {
      dataLib.push({key: TagsLib[i], label: TagsLib[i]});
    }
  }
  view.form.tagLibData = dataLib;
};

const loadData = async () => {
  const res = await GetSettingInfo();
  if (res) {
    view.form = res;
    makeTabLibData();
  }
};

const loadIpAddr = async () => {
  const res = await GetIpAddr();
  view.ipAddr = res.Data;
};

const shutDownHandler = async () => {
  const res = await GetShutDown();
  ElMessage.success(res.Message);
};

onMounted(() => {
  document.title = "設置";
  loadData();
  loadIpAddr();
});
</script>

<style scoped>
.return {
  width: 9%;
  position: fixed;
  bottom: 30px;
  overflow: auto;
  z-index: 999;
  left: 40%;
}

.container {
  margin: 8px 20px;
  width: 90%;
  min-height: 650px;
}

.submit {
  width: 9%;
  position: fixed;
  bottom: 30px;
  overflow: auto;
  z-index: 999;
  right: 40%;
}

.el-transfer {
  margin-left: 120px;
  text-align: left;
}

.inputTag {
  margin: 2px;
}
</style>
