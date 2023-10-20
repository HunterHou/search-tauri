import { axios } from '../../boot/axios';

export const QueryActressList = async (data: unknown) => {
  const res = await axios.post('/api/actressList', data);
  return res;
};
