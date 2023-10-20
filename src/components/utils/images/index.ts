import { useSystemProperty } from '../../../stores/System';
import { computed } from 'vue';

const systemProperty = useSystemProperty();
const settingInfo = computed(() => {
  return systemProperty.SettingInfo;
});

export const getPng = (Id: string) => {
  return settingInfo.value.ImageHost + '/api/png/' + Id;
};
export const getJpg = (Id: string) => {
  return settingInfo.value.ImageHost + '/api/jpg/' + Id;
};

export const getFileStream = (id: string) => {
  return settingInfo.value.StreamHost + '/api/file/' + id;
};

export const getTempImage = (id: string) => {
  return settingInfo.value.StreamHost + '/api/tempimage/' + id;
};

export const getActressImage = (actressUrl: string) => {
  return settingInfo.value.ImageHost + '/api/actressImgae/' + actressUrl;
};
