class FileModel {
  Id = '';
  Tags: string[] | undefined;
  MovieType = '';
  FileType = '';
  Jpg = '';
  Png = '';
  Actress = '';
  Code = '';
  MTime: Date | undefined;
  SizeStr: Date | undefined;
  Name = '';
  Title = '';
  Path = '';
  originName = '';
  listButton: Array<string> | undefined;

  fromObject(data: object) {
    Object.assign(this, data);
    return this;
  }
  isEmpty() {
    return this.Id == undefined || this.Id == null;
  }
}

class FileQuery extends FileModel {
  Page = 1;
  PageSize = 14;
  OnlyRepeat = false;
  Keyword = '';
  SortField = 'MTime';
  SortType = 'desc';
  showStyle = 'post';
}

export { FileQuery, FileModel };
