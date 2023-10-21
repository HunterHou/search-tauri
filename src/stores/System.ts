import { defineStore } from 'pinia';
import { FileModel, FileQuery } from '../components/model/FileModel';
import { SettingInfo } from '../components/model/SettingModel';

export const useSystemProperty = defineStore({
  id: 'system',
  // persist: {
  //   enabled: true,
  //   // 自定义持久化参数
  //   strategies: [
  //     {
  //       // 自定义key
  //       key: "systemProperty",
  //       // 自定义存储方式，默认sessionStorage
  //       storage: localStorage,
  //       // 指定要持久化的数据，默认所有 state 都会进行缓存，可以通过 paths 指定要持久化的字段，其他的则不会进行持久化。
  //     },
  //   ],
  // },
  state: () => ({
    isFullscreen: false,
    shutdownLeftSecond:null,
    shutdownTimer:null,
    Logo: {
      title: 'M系统',
      logo: '',
      url: '/mfilelist',
    },
    videoOptions: {
      autoPlay: true,
      volume: 0.6,
      control: true,
      loop: true,
      muted: true,
      controlBtns: [
        // 'audioTrack',
        'quality',
        'speedRate',
        'volume',
        // 'setting',
        // 'pip',
        'pageFullScreen',
        'fullScreen',
      ],
    },
    History: [],
    Playing: new FileModel(),
    PlayMode: 800,
    drawerRight: false,
    Favorite: [] as FileQuery[],
    FileSearchParam: {
      Page: 1,
      PageSize: 10,
      MovieType: '',
      SortField: 'MTime',
      SortType: 'desc',
      Keyword: '',
      OnlyRepeat: false,
      showStyle: 'post',
      listButton: [
        '播放',
        '编辑',
        '移动',
        '文件夹',
        '转换',
        '删除',
        '剪切',
        '详情',
      ],
    } as FileQuery,
    SettingInfo: {
      ControllerHost: 'http://127.0.0.1:10081',
      ImageHost: 'http://127.0.0.1:10081',
      StreamHost: 'http://127.0.0.1:10081',
    } as SettingInfo,
    SearchSuggestions: [] as Array<string>,
  }),
  getters: {
    getHistory(this) {
      return this.History;
    },
    getFavorite(this) {
      return this.Favorite;
    },
    getSettingInfo(this) {
      return this.SettingInfo;
    },
    getStreamHost(this) {
      return this.SettingInfo.StreamHost;
    },
    getImageHost(this) {
      return this.SettingInfo.ImageHost;
    },
    getControllerHost(this) {
      return this.SettingInfo?.ControllerHost;
    },
    getSuggestions(this) {
      if (!this.SearchSuggestions || this.SearchSuggestions.length == 0) {
        this.SearchSuggestions = JSON.parse(
          localStorage.getItem('SearchSuggestions') || '[]'
        );
      }
      return this.SearchSuggestions;
    },
    getSearchParam(this) {
      return this.FileSearchParam;
    },
  },
  actions: {
    syncSearchParam(param: FileQuery) {
      const {
        Page,
        PageSize,
        MovieType,
        SortField,
        SortType,
        Keyword,
        showStyle,
      } = param;
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
    },
    addFavorite(param: FileQuery) {
      let has = false;
      for (let i = 0; i < this.Favorite.length; i++) {
        const item = this.Favorite[i] as FileQuery;
        if (
          item.Page == param.Page &&
          item.PageSize == param.PageSize &&
          item.Keyword == param.Keyword &&
          item.SortField == param.SortField &&
          item.SortType == param.SortType &&
          item.MovieType == param.MovieType
        ) {
          has = true;
          break;
        }
      }
      if (!has) {
        // this.Favorite.unshift({ ...param });
      }
      if (this.Favorite.length > 50) {
        this.Favorite.splice(0, 49);
      }
    },
    setSettingInfo(settingInfo: SettingInfo) {
      this.SettingInfo = settingInfo;
    },

    setImageHost(url: string) {
      this.SettingInfo.ImageHost = url;
    },
    setStreamHost(url: string) {
      this.SettingInfo.StreamHost = url;
    },
    setControllerHost(url: string) {
      this.SettingInfo.ControllerHost = url;
    },

    setPage(page: number) {
      this.FileSearchParam.Page = page;
    },
    setPageSize(pageSize: number) {
      this.FileSearchParam.PageSize = pageSize;
    },
    setMovieType(MovieType: string) {
      this.FileSearchParam.MovieType = MovieType;
    },
    setKeyword(Keyword: string) {
      this.FileSearchParam.Keyword = Keyword;
    },
    setSortField(SortField: string) {
      this.FileSearchParam.SortField = SortField;
    },
    setSortType(SortType: string) {
      this.FileSearchParam.SortType = SortType;
    },
    setOnlyRepeat(OnlyRepeat: boolean) {
      this.FileSearchParam.OnlyRepeat = OnlyRepeat;
    },

    addSuggestions(queryParam: string) {
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
      localStorage.setItem(
        'SearchSuggestions',
        JSON.stringify(this.SearchSuggestions)
      );
    },
  },
});
