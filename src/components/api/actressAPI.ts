// import { axios } from '../../boot/axios';
import { invoke } from "@tauri-apps/api/tauri";

export const QueryActressList = async (data: unknown) => {
  const res = await invoke("actress_map", {});
  console.log("TypeSizeMap", res);
  return res;
};
