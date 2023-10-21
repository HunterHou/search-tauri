export class SettingInfo {
  Tags: string[] = [];
  ImageTypes: string[] = [];
  DocsTypes: string[] = [];
  VideoTypes: string[] = [];
  DirsLib: string[] = [];
  Dirs: string[] = [];
  Types: string[] = ['mp4', 'jpg'];
  Buttons: string[] = ['刮图', '删除'];
  MovieTypes: string[] = ['骑兵', '步兵', '国产', '漫动'];
  BaseDir: string[] = [];
  tagLibData: string[] = [];
  TagsLib: string[] = [];
  Remark: string | undefined;
  SystemHtml: string | undefined;
  BaseUrl: string | undefined;
  OMUrl: string | undefined;
  ControllerHost: string | undefined;
  ImageHost: string | undefined;
  StreamHost: string | undefined;
  IsDb = false;
}

export const buttonEnum = [
  '播放',
  '删除',
  '移动',
  '编辑',
  '文件夹',
  '详情',
  '刮图',
  '转换',
  '剪切',
  '更多',
];
