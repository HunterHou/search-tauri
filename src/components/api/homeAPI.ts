import { axios } from "../../boot/axios";
import { invoke } from "@tauri-apps/api/tauri";

export const TypeSizeMap = async () => {
  const res = await invoke("type_size_map", {});
  console.log("TypeSizeMap", res);
  return res;
};

export const TagSizeMap = async () => {
  const res = await axios.get("/api/tagSizeMap");
  return res && res.data;
};

export const ScanTime = async () => {
  const res = await axios.get("/api/scanTime");
  return res && res.data;
};
