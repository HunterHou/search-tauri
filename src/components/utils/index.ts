export const formatMovieType = (title: string) => {
  if (title.indexOf('{{') >= 0) {
    title = title.split('{{')[1];
  }
  if (title.indexOf('}}') >= 0) {
    title = title.split('}}')[0];
  }
  return title;
};

export const formatTags = (title: string) => {
  if (title.indexOf('《') >= 0) {
    title = title.split('《')[1];
  }
  if (title.indexOf('》') >= 0) {
    title = title.split('》')[0];
  }
  return title;
};

export const formatCode = (code: string | undefined) => {
  if (code) {
    if (code.indexOf('-') == 0) {
      return (code = code.substring(1));
    }
    return code.substring(0, 10);
  }
  return '';
};

export const formatTitle = (title: string | undefined) => {
  if (!title) {
    return '';
  }
  if (title.lastIndexOf(']') >= 0) {
    title = title.substring(title.lastIndexOf(']') + 1);
  }
  if (title.indexOf('{{') >= 0) {
    title = title.replace(`{{${formatMovieType(title)}}}`, '');
  }
  if (title.indexOf('《') >= 0) {
    title = title.replace(`《${formatTags(title)}》`, '');
  }
  return title;
};

export const MovieTypeOptions = [
  { label: '骑兵', value: '骑兵' },
  { label: '步兵', value: '步兵' },
  { label: '国产', value: '国产' },
  { label: '斯巴达', value: '斯巴达' },
  { label: '漫动', value: '漫动' },
];

export const MovieTypeSelects = [
  { label: '全部', value: '' },
  ...MovieTypeOptions,
  { label: '无', value: '无' },
];
