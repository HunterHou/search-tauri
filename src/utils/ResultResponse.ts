export class ResultResponse {
  code!: string;
  msg!: string;
  data: any;
  constructor() {}
}

export class ResultList {
  Data: any;
  TotalCnt!: string;
  ResultCnt!: string;
  CurCnt!: string;
  TotalPage!: string;
  TotalSize!: string;
  ResultSize!: string;
  CurSize!: string;
}
