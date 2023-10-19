import { useSystemProperty } from "@/store/System";
import { reactive } from "vue";

const systemProperty = useSystemProperty();

export const formItemTagsChange = (view) => {
  let { Name, Tags, FileType, Code, Actress } = view.formItem;
  let newName = "";
  if (Name.indexOf("《") >= 0) {
    const startC = Name.substr(0, Name.indexOf("《") + 1);
    const endC = Name.substr(Name.indexOf("》"), Name.length);
    newName = startC;
    if (Tags && Tags.length > 0) {
      newName += Tags;
    }
    newName += endC;
  } else {
    newName = Name.replaceAll("." + FileType, "");
    newName = newName + "《" + Tags + "》" + "." + FileType;
  }
  view.formItem.originName = newName;
};

export const formMovieTypeChange = (view: any) => {
  if (!view) {
    return;
  }
  let { MovieType, Name, FileType } = view.currentFile || {};
  let newName = "";
  if (Name.indexOf("{{") >= 0) {
    const startC = Name.substr(0, Name.indexOf("{{"));
    const endC = Name.substr(Name.indexOf("}}") + 2, Name.length);
    newName = startC;
    if (MovieType && MovieType !== "") {
      newName += "{{" + MovieType + "}}";
    }
    newName += endC;
  } else {
    newName = Name.replaceAll("." + FileType, "");
    newName = newName + "{{" + MovieType + "}}" + "." + FileType;
  }
  view.currentFile.Name = newName;
  view.currentFile.originName = newName;
};

export const javSearch = (actress: string) => {
  const url = systemProperty.getSettingInfo.BaseUrl + "/search/" + actress;
  window.open(url);
};
export const javCode = (code: string) => {
  const url = systemProperty.getSettingInfo.BaseUrl + "/" + code;
  window.open(url);
};

export const fetchSuggestion = (queryString: string, callback) => {
  const sourceSuggestions = systemProperty.getSuggestions;
  if (!sourceSuggestions) {
    return;
  }
  const results = queryString
    ? sourceSuggestions.filter(createFilter(queryString))
    : sourceSuggestions;
  // 调用 callback 返回建议列表的数据
  const finalResults = results.slice(0, 50);
  callback(finalResults);
};

export const fetchTagsLib = (queryString, callback) => {
  const suggrestTagsLib = systemProperty.getSettingInfo?.TagsLib;
  const results = queryString
    ? suggrestTagsLib.filter(createFilter(queryString))
    : suggrestTagsLib;
  callback(results);
};

const createFilter = (queryString) => {
  return (res) => {
    return res.toLowerCase().indexOf(queryString.toLowerCase()) >= 0;
  };
};

export const notContainTag = (tags: string[], tag: string) => {
  if (!tags || !tag) {
    return true;
  }
  if (tags.indexOf(tag) < 0) {
    return true;
  }
  return false;
};
export const customerTagEmpty = (view: any) => {
  if (view.customerTag) {
    return false;
  }
  return true;
};

export const notQiBing = (movieType: string): boolean => {
  if (movieType !== "骑兵") {
    return true;
  }
  return false;
};

export const notSiBaDa = (movieType: string): boolean => {
  if (movieType !== "斯巴达") {
    return true;
  }
  return false;
};
export const notNative = (movieType: string): boolean => {
  if (movieType !== "国产") {
    return true;
  }
  return false;
};

export const notBuBing = (movieType: string): boolean => {
  if (movieType !== "步兵") {
    return true;
  }
  return false;
};
export const volumechange = (e) => {
  const {
    target: { volume ,muted},
  } = e;
  systemProperty.videoOptions.volume = volume;
  systemProperty.videoOptions.muted = muted;
};
