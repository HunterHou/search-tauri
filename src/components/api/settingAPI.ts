import { axios } from '../../boot/axios';
import { SettingInfo } from '../model/Setting';

export const GetSettingInfo = async () => {
  const res = await axios.get('/api/buttoms');
  return res;
};

export const PostSettingInfo = async (data: SettingInfo) => {
  const res = await axios.post('/api/setting', data);
  return res && res.data;
};

export const GetIpAddr = async () => {
  const res = await axios.get('/api/GetIpAddr');
  return res && res.data;
};

export const GetShutDown = async () => {
  const res = await axios.get('/api/shutDown');
  return res as unknown;
};
