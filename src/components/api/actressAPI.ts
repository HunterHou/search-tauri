import { api } from '../../boot/axios';

export const QueryActressList = async (data: unknown) => {
  const res = await api.post('/api/actressList', data);
  return res;
};
