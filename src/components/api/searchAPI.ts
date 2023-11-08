import { axios } from "../../boot/axios";
import { invoke } from "@tauri-apps/api/tauri";
import sys from "../utils/system";
import { FileModel } from "../model/FileModel";

export const SearchAPI = async (params: any) => {
  // console.log('SearchAPI params',params);
  const data = await invoke("search_index", {
    params: JSON.stringify({
      ...params,
      params: { ...params },
      Keyword: params.Keyword || "",
    }),
  });
  // console.log('SearchAPI data',data);
  return data;
};

export const RefreshAPI = async () => {
  const res = await invoke("refresh_disk", {
    name: "refresh_disk",
  });
  return res;
};

export const FindFileInfo = async (data: string) => {
  const res = await invoke("find_file_info", {
    id: data,
  });
  return res;
};

export const QueryDirImageBase64 = async (data: string) => {
  const res = await invoke("files_by_dir", {
    path: data,
  });
  return res;
};

export const DeleteFolerByPath = async (data: string) => {
  sys.DeleteDir({ Path: data });
  return { code: 200 };
};

export const DeleteFile = async (data: FileModel) => {
  const { Path, Jpg, Png, Gif } = data;
  sys.DeleteFile({ Path: Path });
  sys.DeleteFile({ Path: Jpg });
  sys.DeleteFile({ Path: Png });
  sys.DeleteFile({ Path: Gif });
  return { code: 200 };
};

export const FileRename = async (data: FileModel) => {
  console.log("FileRename1", data);
  const e1 = await sys.ExistsFile(data.Path)
  console.log('e1',e1)
  console.log("ExistsFile(data.Path)",e1 );
  const e2 = await sys.ExistsFile(data.Jpg)
  console.log("ExistsFile(data.Jpg)", e2);
  console.log("ExistsFile(data.Png)", await sys.ExistsFile(data.Png));
  console.log("ExistsFile(data.Gif)", await sys.ExistsFile(data.Gif));
};

export const AddTag = async (clickId: string, title: string) => {
  const res = await axios.get(`/api/file/addTag/${clickId}/${title}`);
  return res && res.data;
};

export const CloseTag = async (id: string, title: string) => {
  const res = await axios.get(`/api/file/clearTag/${id}/${title}`);
  return res && res.data;
};

export const ResetMovieType = async (data: string, movieType: string) => {
  const res = await axios.get(`/api/setMovieType/${data}/${movieType}`);
  return res && res.data;
};

export const TransferTasksInfo = async () => {
  const res = await axios.get("/api/transferTasks");
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

export const DownImageList = async (data: string): Promise<unknown> => {
  const res = await axios.get(`/api/imageList/${data}`);
  return res && res.data;
};
