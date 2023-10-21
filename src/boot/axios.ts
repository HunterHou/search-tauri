// import { boot } from 'quasar/wrappers';
// import axios, { AxiosInstance } from 'axios';

// declare module '@vue/runtime-core' {
//   interface ComponentCustomProperties {
//     $axios: AxiosInstance;
//     $api: AxiosInstance;
//   }
// }

// // Be careful when using SSR for cross-request state pollution
// // due to creating a Singleton instance here;
// // If any client changes this (global) instance, it might be a
// // good idea to move this instance creation inside of the
// // "export default () => {}" function below (which runs individually
// // for each client)
// const api = axios.create({ baseURL: 'http://localhost:10081',headers:{"Access-Control-Allow-Origin":"*"} });

// export default boot(({ app }) => {
//   // for use inside Vue files (Options API) through this.$axios and this.$api

//   app.config.globalProperties.$axios = axios;
//   // ^ ^ ^ this will allow you to use this.$axios (for Vue Options API form)
//   //       so you won't necessarily have to import axios in each vue file

//   app.config.globalProperties.$api = api;
//   // ^ ^ ^ this will allow you to use this.$api (for Vue Options API form)
//   //       so you can easily perform requests against your app's API
// });

import { fetch ,Body} from '@tauri-apps/api/http';

class Http {
  BaseUrl ="http://localhost:10081"
  async get(url:string,params?:any) {
  const res =  await fetch(this.BaseUrl+url, {
      headers:{
          Authorization: 'Bearer test'
      },
      method: 'GET',
      query: params
  })
  return res
  }
  async post(url:string,data?:any){
    const res =  await fetch(this.BaseUrl+url, {
      headers:{
          Authorization: 'Bearer test'
      },
      method: 'POST',
      // 常规的json格式请求体发送
      body: Body.json(data)
  })
  return res
  }
}
const axios = new Http()
export { axios };
