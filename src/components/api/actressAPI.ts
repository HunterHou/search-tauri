// import { axios } from '../../boot/axios';
import { invoke } from "@tauri-apps/api/tauri";

export const QueryActressList = async (data: unknown) => {
  console.log("QueryActressList params", data);
  const res = await invoke("actress_map", { params:JSON.stringify(data)});
  console.log("QueryActressList res", res);
  return res;
};
