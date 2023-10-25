import { axios } from '../../boot/axios';
import { invoke } from '@tauri-apps/api/tauri'

export const SearchAPI = async (params: object) => {
  // const { data } = await axios.post('/api/movieList', params);
  // return data;

  const params1 = { ...params, params: {...params}, FileType: ["mp4", "mkv"] }
  console.log(params);
  const data = await invoke("search_index", { params: JSON.stringify(params1) })
  return data;
};

export const RefreshAPI = async (params: object) => {
  const res = await axios.get('/api/refreshIndex', params);
  return res && res.data;
};

export const FindFileInfo = async (data: string) => {
  const res = await axios.get(`/api/info/${data}`);
  return res&&res.data;
};

export const QueryDirImageBase64 = async (data: string) => {
  const res = await axios.get(`/api/dir/${data}`);
  return res;
};

export const PlayMovie = async (data: string) => {
  const res = await axios.get(`/api/play/${data}`);
  return res && res.data;
};

export const OpenFileFolder = async (data: string) => {
  const res = await axios.get(`/api/openFolder/${data}`);
  return res && res.data;
};

export const DeleteFile = async (data: string) => {
  const res = await axios.get(`/api/delete/${data}`);
  return res && res.data;
};

export const SyncFileInfo = async (data: object) => {
  const res = await axios.post('/api/sync',data);
  return res && res.data;
};

export const TransferTasksInfo = async () => {
  const res = await axios.get('/api/transferTasks');
  return res && res.data;
};

export const TansferFile = async (data: string) => {
  const res = await axios.get(`/api/tranferToMp4/${data}`);
  return res && res.data;
};

export const CutFile = async (id: string, start: string, end: string) => {
  const res = await axios.get(`/api/cutMovie/${id}/${start}/${end}`);
  return res && res.data;
};

export const ResetMovieType = async (data: string, movieType: string) => {
  const res = await axios.get(`/api/setMovieType/${data}/${movieType}`);
  return res && res.data;
};

export const DownImageList = async (data: string): Promise<unknown> => {
  const res = await axios.get(`/api/imageList/${data}`);
  return res && res.data;
};

export const HeartBeatQuery = async () => {
  const res = await axios.get('/api/heartBeat');
  return res && res.data;
};

export const AddTag = async (clickId: string, title: string) => {
  const res = await axios.get(`/api/file/addTag/${clickId}/${title}`);
  return res && res.data;
};

export const CloseTag = async (id: string, title: string) => {
  const res = await axios.get(`/api/file/clearTag/${id}/${title}`);
  return res && res.data;
};

export const FileRename = async (data: unknown) => {
  const res = await axios.post('/api/file/rename', data);
  return res && res.data;
};

export const OpenFolerByPath = async (data: unknown) => {
  const res = await axios.post('/api/OpenFolerByPath', data);
  return res && res.data;
};
export const DeleteFolerByPath = async (data: unknown) => {
  const res = await axios.post('/api/DeleteFolerByPath', data);
  return res && res.data;
};


