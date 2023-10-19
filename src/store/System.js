import { defineStore } from "pinia";
import { store } from "./pinia";

const SystemProperty = defineStore({
  id: "system",
  persist: {
    enabled: true,
    // 自定义持久化参数
    strategies: [
      {
        // 自定义key
        key: "systemProperty",
        // 自定义存储方式，默认sessionStorage
        storage: localStorage,
        // 指定要持久化的数据，默认所有 state 都会进行缓存，可以通过 paths 指定要持久化的字段，其他的则不会进行持久化。
      },
    ],
  },
  state: () => ({
    isFullscreen:false,
    Logo: {
      title: "M系统",
      logo: "",
      url: "/mfilelist",
    },
    videoOptions: {
      autoPlay: true,
      volume: 0.6,
      control: true,
      loop: true,
      muted: true,
      controlBtns: [
        "audioTrack",
        "quality",
        "speedRate",
        "volume",
        "setting",
        "pip",
        "pageFullScreen",
        "fullScreen",
      ],
    },
    History: [],
    Favorite: [],
    FileSearchParam: {
      Page: 1,
      PageSize: 14,
      MovieType: "",
      SortField: "MTime",
      SortType: "desc",
      Keyword: "",
      OnlyRepeat: false,
      showStyle: 'post',
    },
    SettingInfo: {
      ControllerHost: "http://127.0.0.1:8081",
      ImageHost: "http://127.0.0.1:8081",
      StreamHost: "http://127.0.0.1:8081",
    },
    SearchSuggestions: [],
  }),
  getters: {
    getHistory() {
      return this.History;
    },
    getFavorite() {
      return this.Favorite;
    },
    getSettingInfo() {
      return this.SettingInfo;
    },
    getStreamHost() {
      return this.StreamHost;
    },
    getImageHost() {
      return this.ImageHost;
    },
    getControllerHost() {
      return this.SettingInfo?.ControllerHost;
    },
    getSuggestions() {
      return this.SearchSuggestions;
    },
    getSearchParam() {
      return this.FileSearchParam;
    },
  },
  actions: {
    syncSearchParam(param) {
      const { Page, PageSize, MovieType, SortField, SortType, Keyword, showStyle } = param;
      this.FileSearchParam.Page = Page;
      this.FileSearchParam.PageSize = PageSize;
      this.FileSearchParam.MovieType = MovieType;
      this.FileSearchParam.SortField = SortField;
      this.FileSearchParam.SortType = SortType;
      this.FileSearchParam.Keyword = Keyword;
      this.FileSearchParam.showStyle = showStyle;
      if (param.Keyword) {
        this.addSuggestions(param.Keyword);
      }
      this.addHistory(param);
    },
    addHistory(his) {
      let has = false;
      for (let i = 0; i < this.History.length; i++) {
        if (
          this.History[i].Page == his.Page &&
          this.History[i].PageSize == his.PageSize &&
          this.History[i].Keyword == his.Keyword &&
          this.History[i].SortField == his.SortField &&
          this.History[i].SortType == his.SortType &&
          this.History[i].MovieType == his.MovieType
        ) {
          has = true;
          break;
        }
      }
      if (!has) {
        this.History.unshift({ ...his, createTime: new Date() });
      }
      if (this.History.length > 50) {
        this.History.splice(0, 49);
      }
    },
    addFavorite(his) {
      let has = false;
      for (let i = 0; i < this.Favorite.length; i++) {
        if (
          this.Favorite[i].Page == his.Page &&
          this.Favorite[i].PageSize == his.PageSize &&
          this.Favorite[i].Keyword == his.Keyword &&
          this.Favorite[i].SortField == his.SortField &&
          this.Favorite[i].SortType == his.SortType &&
          this.Favorite[i].MovieType == his.MovieType
        ) {
          has = true;
          break;
        }
      }
      if (!has) {
        this.Favorite.unshift({ ...his });
      }
      if (this.Favorite.length > 50) {
        this.Favorite.splice(0, 49);
      }
    },
    setSettingInfo(settingInfo) {
      this.SettingInfo = settingInfo;
    },

    setImageHost(url) {
      this.SettingInfo.ImageHost = url;
    },
    setStreamHost(url) {
      this.SettingInfo.StreamHost = url;
    },
    setControllerHost(url) {
      this.SettingInfo.ControllerHost = url;
    },

    setPage(page) {
      this.FileSearchParam.Page = page;
    },
    setPageSize(pageSize) {
      this.FileSearchParam.PageSize = pageSize;
    },
    setMovieType(MovieType) {
      this.FileSearchParam.MovieType = MovieType;
    },
    setKeyword(Keyword) {
      this.FileSearchParam.Keyword = Keyword;
    },
    setSortField(SortField) {
      this.FileSearchParam.SortField = SortField;
    },
    setSortType(SortType) {
      this.FileSearchParam.SortType = SortType;
    },
    setOnlyRepeat(OnlyRepeat) {
      this.FileSearchParam.OnlyRepeat = OnlyRepeat;
    },

    addSuggestions(queryParam) {
      if (!queryParam) {
        return;
      }
      if (!this.SearchSuggestions) {
        this.SearchSuggestions = [];
      }
      const idx = this.SearchSuggestions.indexOf(queryParam);
      if (idx >= 0) {
        this.SearchSuggestions.splice(idx, 1);
      }
      this.SearchSuggestions.unshift(queryParam);
      if (this.SearchSuggestions.length > 100) {
        this.SearchSuggestions.pop();
      }
    },
  },
});
export const useSystemProperty = () => {
  return SystemProperty(store);
};
