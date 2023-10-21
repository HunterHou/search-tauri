<template>
  <q-dialog ref="dialogRef" class="card-q" @hide="dialogHide" @before-show="beforeShow">
    <q-card class="card-q">
      <q-tabs v-model="tab" class="bg-purple text-white" style="" align="justify" narrow-indicator>
        <q-tab name="filelist" label="批量操作" />
        <q-tab name="setting" label="设置" />
        <q-tab name="tasking" label="任务执行" />
      </q-tabs>
      <q-separator />
      <q-tab-panels v-model="tab" animated class="bg-purple-1" style="height: 100%; overflow: auto">
        <q-tab-panel name="filelist">
          <div class="q-mr-sm q-mb-sm  row justify-left">
            <q-btn-toggle v-model="view.queryParam.MovieType" @update:model-value="fetchSearch()" toggle-color="primary"
              :options="MovieTypeSelects" />
            <q-btn class="q-mr-sm" v-if="view.queryParam.Page != 1" size="sm" color="secondary"
              @click="nextPage(-1)">上</q-btn>
            <q-btn class="q-mr-sm" size="sm" color="secondary" @click="nextPage(1)">下</q-btn>

          </div>
          <div class="q-mr-sm row justify-left">
            <q-btn class=" q-mr-sm" color="amber" size="sm" glossy text-color="black" @click="selectAll">{{
              view.selectAll
              ? '不选' : '全选' }}</q-btn>
            <q-input label="..." v-model="view.queryParam.Keyword" :dense="true" filled clearable
              @update:model-value="fetchSearch()" />
            <q-btn class="q-mr-sm" size="sm" color="secondary" icon="refresh" @click="refreshIndex">刷新</q-btn>
            <q-btn-dropdown class="q-mr-sm" size="sm" label="设置" type="primary" color="teal" icon="ti-settings">
              <q-list>
                <q-item v-for="mt in MovieTypeOptions" :key="mt.value" v-close-popup class="movieTypeSelectItem">
                  <q-item-section @click="setTypeBySelector(mt.value)">
                    <q-item-label>{{ mt.label }}
                    </q-item-label>
                  </q-item-section>
                </q-item>
              </q-list>
            </q-btn-dropdown>
            <q-btn-dropdown class="q-mr-sm" size="sm" label="标签" type="primary" color="teal" icon="ti-plus">
              <div style="width: 300px;">
                <q-btn size="sm" icon='ti-plus' square text-color="white" color="red" class="tag-item"
                  v-for="tag in  view.settingInfo.Tags" :key="tag" :label="tag" @click="addTagBySelector(tag)" />
              </div>
            </q-btn-dropdown>
            <q-btn class="q-mr-sm" size="sm" color="secondary" icon="delete" @click="deleteBySelector">删除</q-btn>
          </div>
          <div class="q-gutter-sm q-mt-sm">
            <div v-ripple v-for="item in view.resultData.Data" :key="item.Id" style="
                border: 1px dotted purple;
                padding: 2px;
                border-radius: 10px;
                background-color: burlywood;
              ">
              <div style="display: flex; flex-direction: column">
                <q-item-label>
                  <span v-if="view.cutListIds.indexOf(item.Id) >= 0" style="color: red">剪切中：：</span>{{ item.Title }}【{{
                    item.SizeStr }}】
                </q-item-label>
                <div style="display: flex; flex-direction: row">
                  <q-checkbox size="sm" v-model="view.selector" :val="item.Id" color="red" />
                  <q-btn-dropdown class="q-mr-sm" size="sm" :label="item.MovieType" type="primary" color="teal">
                    <q-list>
                      <q-item v-for="mt in MovieTypeOptions" :key="mt.value" v-close-popup class="movieTypeSelectItem">
                        <q-item-section>
                          <q-item-label @click="
                            item.MovieType = mt.value;
                          commonExec(ResetMovieType(item.Id, mt.value));
                          ">{{ mt.label }}
                          </q-item-label>
                        </q-item-section>
                      </q-item>
                    </q-list>
                  </q-btn-dropdown>
                  <q-btn size="sm" class="q-mr-sm" color="amber" glossy text-color="black" icon="delete"
                    @click="confirmDelete(item)" />
                  <q-btn size="sm" class="q-mr-sm" color="black-1" @click="moveThis(item)" icon="near_me" />
                  <q-btn class="q-mr-sm" size="sm" color="secondary" icon="open_in_new"
                    @click="commonExec(OpenFileFolder(item.Id))" />
                  <q-btn size="sm" class="q-mr-sm" color="black" icon="ti-pencil-alt2"
                    @click="item.showCut = true"></q-btn>
                  <q-btn size="sm" class="q-mr-sm" color="green" @click="toMp4(item)">toMp4</q-btn>
                  <q-btn class="q-mr-sm" size="sm" color="brown-5" icon="wifi_protected_setup"
                    v-if="!item.MovieType || item.MovieType == '无'" @click="commonExec(SyncFileInfo(item.Id))" />
                  <q-btn color="red" v-for="ta in item.Tags" :key="ta" @click="commonExec(CloseTag(item.Id, ta), true)">{{
                    `- ${ta}` }}</q-btn>
                </div>
                <div v-if="item.showCut" style="
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    background-color: antiquewhite;
                    border-radius: 12px;
                    padding: 4px;
                  ">
                  开始：
                  <q-input style="width: 40px" v-model:model-value="cutParam.sH"></q-input>
                  <q-input style="width: 40px" v-model:model-value="cutParam.sM"></q-input>
                  <q-input style="width: 40px" v-model:model-value="cutParam.sS"></q-input>
                  结束：
                  <q-input style="width: 40px" v-model:model-value="cutParam.eH"></q-input>
                  <q-input style="width: 40px" v-model:model-value="cutParam.eM"></q-input>
                  <q-input style="width: 40px" v-model:model-value="cutParam.eS"></q-input>
                  <q-btn size="sm" color="black" type="primary" @click="
                    cutThis(item);
                  item.showCut = false;
                  " label="确认" />
                  <q-btn size="sm" color="blue" @click="item.showCut = false" label="取消" />
                </div>
              </div>
            </div>
          </div>
        </q-tab-panel>
        <q-tab-panel name="setting" class="bg-purple-2">
          <q-field color="purple-12" label="Buttons（最佳5）" stack-label>
            <template v-slot:prepend>
              <q-icon name="event" />
            </template>
            <template v-slot:control>
              <q-checkbox v-model="view.settingInfo.Buttons" v-for="item in buttonEnum" :key="item" :val="item"
                :label="item" color="teal" @update:model-value="updateButtons" />
            </template>
          </q-field>
        </q-tab-panel>

        <q-tab-panel name="tasking">
          <q-list bordered separator>
            <q-expansion-item v-for="(v, k) in view.tasking" :key="k">
              <template v-slot:header>
                <q-item-section avatar>
                  <q-btn color="blue" class="q-mr-sm">{{ v.Type }}</q-btn>
                  {{ ` 格式：${v.To} ` }}
                </q-item-section>
                <q-item-section>
                  <div>{{ v.Name }}</div>
                  <div>
                    {{ `开始时间：${v.Start} ` }}
                    {{ ` 结束时间：${v.End} ` }}
                  </div>
                  <div>
                    {{
                      `耗时：${(new Date(v.FinishTime).getTime() -
                        new Date(v.CreateTime).getTime()) /
                        1000
                        }`
                    }}
                  </div>
                </q-item-section>
                <q-item-section side>
                  <q-btn class="q-mr-sm" :color="v.Status == '成功' ? 'green' : 'black'">{{ v.Status }}
                  </q-btn>
                </q-item-section>
              </template>
              <q-card>
                <q-card-section>
                  <div style="max-height: 20vh; overflow: auto">
                    {{ v.Log }}
                  </div>
                </q-card-section>
              </q-card>
            </q-expansion-item>
          </q-list>
        </q-tab-panel>
      </q-tab-panels>
    </q-card>
  </q-dialog>
</template>

<script setup>
import { useQuasar } from 'quasar';
import { useDialogPluginComponent } from 'quasar';
import { reactive, ref, watch } from 'vue';

import { MovieTypeOptions } from '@/components/utils';
import { buttonEnum } from '@/components/model/Setting';
import { MovieTypeSelects } from '@/components/utils';
import {
  ResetMovieType,
  SearchAPI,
  RefreshAPI,
  FileRename,
  DeleteFile,
  SyncFileInfo,
  CutFile,
  TransferTasksInfo,
  TansferFile,
  CloseTag,
  AddTag
} from '@/components/api/searchAPI';
import { PostSettingInfo } from '@/components/api/settingAPI';

const $q = useQuasar();

const tab = ref('filelist');
let timeFunc;
watch(
  () => tab.value,
  (v) => {
    console.log(v);
    if (v === 'tasking') {
      fetchTasking();
      timeFunc = setInterval(fetchTasking, 2000);
    } else {
      clearInterval(timeFunc);
    }
  }
);

const fetchTasking = async () => {
  const res = await TransferTasksInfo();
  view.tasking = res.Data;
};
const view = reactive({
  selectAll: false,
  settingInfo: {},
  resultData: {},
  queryParam: {},
  selector: [],
  callback: null,
  cutListIds: [],
  tasking: {}
});
const cutParam = reactive({
  sH: '00',
  sM: '00',
  sS: '00',
  eH: '99',
  eM: '00',
  eS: '00'
});

const toMp4 = (item) => {
  if (view.cutListIds.indexOf(item.Id) < 0) {
    view.cutListIds.push(item.Id);
  }
  commonExec(TansferFile(item.Id));
};


const cutThis = (item) => {
  if (view.cutListIds.indexOf(item.Id) < 0) {
    view.cutListIds.push(item.Id);
  }
  commonExec(
    CutFile(
      item.Id,
      [cutParam.sH, cutParam.sM, cutParam.sS].join(':'),
      [cutParam.eH, cutParam.eM, cutParam.eS].join(':')
    )
  );
};

defineEmits([
  // REQUIRED; 需要明确指出
  // 组件通过 useDialogPluginComponent() 暴露哪些事件
  ...useDialogPluginComponent.emits
]);

const selectAll = () => {
  view.selectAll = !view.selectAll;
  if (view.selectAll) {
    view.selector = view.resultData.Data.map((item) => item.Id);
  } else {
    view.selector = [];
  }
};

const setTypeBySelector = (value) => {
  if (view.selector && view.selector.length > 0) {
    view.selector.forEach(item => {
      commonExec(ResetMovieType(item, value));
    })
  }
}
const deleteBySelector = () => {
  if (view.selector && view.selector.length > 0) {
    view.selector.forEach(item => {
      commonExec(DeleteFile(item))
    })
  }
}

const addTagBySelector = (value) => {
  if (view.selector && view.selector.length > 0) {
    view.selector.forEach(item => {
      commonExec(AddTag(item, value));
    })
  }
}


const refreshIndex = async () => {
  await RefreshAPI();
  await fetchSearch();
};

const nextPage = (n) => {
  view.queryParam.Page = view.queryParam.Page + n
  fetchSearch()
}

const fetchSearch = async () => {
  const data = await SearchAPI(view.queryParam);
  view.resultData = data;
};

const commonExec = async (exec) => {
  const { Code, Message } = await exec;
  console.log(Code, Message);
  if (Code != 200) {
    $q.notify({ type: 'positive', message: `${Message}`, multiLine: true, position: 'bottom-right', multiLine: true, position: 'bottom-right' });
  } else {
    $q.notify({ type: 'negative', message: `${Message}`, multiLine: true, position: 'bottom-right', multiLine: true, position: 'bottom-right' });
  }
};

const open = (data) => {
  const { queryParam, settingInfo, cb } = data;
  view.queryParam = queryParam;
  view.queryParam.PageSize = 10
  view.settingInfo = settingInfo;
  view.callback = cb;
  dialogRef.value.show();
  fetchSearch();
};

const dialogHide = async () => {
  if (view.callback) {
    view.callback({ settingInfo: view.settingInfo });;
  }
  onDialogCancel();
  onDialogOK();
  onDialogHide();
  console.log('dialogHide')
};

const confirmDelete = (item) => {
  $q.dialog({
    title: item.Name,
    message: '确定删除吗?',
    cancel: true,
    persistent: true
  })
    .onOk(() => {
      console.log('>>>> onOk');
      commonExec(DeleteFile(item.Id)).then(() => {
        for (let i = 0; i < view.resultData.Data.length; i++) {
          if (view.resultData.Data[i].Id == item.Id) {
            view.resultData.Data.splice(i, 1);
          }
        }
      });
    })
    .onCancel(() => {
      console.log('>>>> Cancel');
    })
    .onDismiss(() => {
      // console.log('I am triggered on both OK and Cancel')
    });
};

const moveThis = async (item) => {
  const res = await FileRename({ ...item, NoRefresh: true, MoveOut: true });
  console.log(res);
  if (res.Code == 200) {
    for (let i = 0; i < view.resultData.Data.length; i++) {
      if (view.resultData.Data[i].Id == item.Id) {
        view.resultData.Data.splice(i, 1);
      }
    }
    $q.notify({ type: 'positive', message: res.Message, multiLine: true, position: 'bottom-right' });
  } else {
    $q.notify({ type: 'negative', message: res.Message, multiLine: true, position: 'bottom-right' });
  }
};

const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } =
  useDialogPluginComponent();
// dialogRef      - 用在 QDialog 上的 Vue ref 模板引用
// onDialogHide   - 处理 QDialog 上 @hide 事件的函数
// onDialogOK     - 对话框结果为 ok 时会调用的函数
//                    示例: onDialogOK() - 不带参数
//                    示例: onDialogOK({ /*.../* }) - 带参数
// onDialogCancel - 对话框结果为 cancel 时调用的函数

// 这是示例的内容，不是必需的
// const onOKClick = () => {
// REQUIRED！ 对话框的结果为 ok 时，必须调用 onDialogOK()  (参数是可选的)
// onDialogOK()
// 带参数的版本: onDialogOK({ ... })
// ...会自动关闭对话框
// }

const updateButtons = () => {
  if (view.callback) {
    PostSettingInfo(view.settingInfo)
    view.callback({ settingInfo: view.settingInfo });
  }
};

const beforeShow = () => {
  console.log('beforeShow');
};

defineExpose({
  open
});
</script>

<style>
.card-q {
  display: flex;
  flex-direction: column;
  height: 80vh;
  width: 70vw !important;
  max-width: 70vw !important;
}

.tag-item {
  margin: 2px 4px;
  padding: 1px 6px;
  border-radius: 8px;
}
</style>
