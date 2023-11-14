import { axios } from "../../boot/axios";
import { invoke } from "@tauri-apps/api/tauri";
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
  const res = await invoke("delete_dir", {
    path: data,
  });
  return res;
};

export const DeleteFile = async (data: FileModel) => {
  const res = await invoke("delete_model", {
    path: data.Id,
  });
  return res;
};

export const FileRename = async (data: any) => {
  const res = await invoke("rename_model", {
    isMove: data.MoveOut,
    params: JSON.stringify(data),
  });
  return res;
};

export const AddTag = async (clickId: string, tag: string) => {
  const res = await invoke("add_tag", {
    id: clickId,
    tag: tag,
  });
  return res;
};

export const CloseTag = async (clickId: string, tag: string) => {
  const res = await invoke("remove_tag", {
    id: clickId,
    tag: tag,
  });
  return res;
};

export const ResetMovieType = async (clickId: string, movieType: string) => {
  const res = await invoke("set_movie_type", {
    id: clickId,
    movieType,
  });
  return res;
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
