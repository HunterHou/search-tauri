import { api } from '../../boot/axios';
import { SettingInfo } from '../model/SettingModel';

export const GetSettingInfo = async () => {
  const res = await api.get('/api/buttoms');
  return res;
};

export const PostSettingInfo = async (data: SettingInfo) => {
  const res = await api.post('/api/setting', data);
  return res && res.data;
};

export const GetIpAddr = async () => {
  const res = await api.get('/api/GetIpAddr');
  return res && res.data;
};

export const GetShutDown = async () => {
  const res = await api.get('/api/shutDown');
  return res as unknown;
};
