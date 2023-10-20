class MovieModel {
  Id!: string;
  Tags!: string[];
  MovieType!: string;
  FileType!: string;
  Jpg!: string;
  Png!: string;
  Actress!: string;
  Code!: string;
  MTime!: Date;
  SizeStr!: Date;
  Name!: string;
  originName!: string;
}

class MovieQuery extends MovieModel {
  Page = 1;
  PageSize = 14;
  OnlyRepeat = false;
  Keyword!: string;
  SortField = "MTime";
  SortType = "desc";
  showStyle = "post";
}

export { MovieQuery, MovieModel };
