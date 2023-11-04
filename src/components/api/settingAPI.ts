import { axios } from "../../boot/axios";
import { SettingInfo } from "../model/SettingModel";
import { invoke } from "@tauri-apps/api/tauri";

export const GetSettingInfo = async () => {
  const res = await invoke("read_settings", {});
  console.log("GetSettingInfo", res);
  return res;
};

export const PostSettingInfo = async (data: SettingInfo) => {
  const res = await invoke("submit_settings", {
    params: JSON.stringify(data),
  });
  return res;
};

export const GetIpAddr = async () => {
  const res = await axios.get("/api/GetIpAddr");
  return res && res.data;
};

export const GetShutDown = async () => {
  const res = await axios.get("/api/shutDown");
  return res as unknown;
};
