import axios, { AxiosRequestConfig, AxiosError, AxiosResponse } from "axios";
import { ElNotification } from "element-plus";
import qs from "qs";
import { config } from "./config";
const server = axios.create({ baseURL: "", timeout: config.request_timeout });
server.interceptors.request.use(
  (config: AxiosRequestConfig) => {
    if (
      config.method === "post" &&
      config!.headers!["Content-Type"] === "application/x-www-form-urlencoded"
    ) {
      config.data = qs.stringify(config.data);
    }
    // 添加token，可根据实际业务修改
    // config!.headers!['Authorization'] = 'something'
    // get参数编码
    if (config.method === "get" && config.params) {
      let url = config.url as string;
      url += "?";
      const keys = Object.keys(config.params);
      for (const key of keys) {
        if (config.params[key] !== void 0 && config.params[key] !== null) {
          url += `${key}=${encodeURIComponent(config.params[key])}&`;
        }
      }
      url = url.substring(0, url.length - 1);
      config.params = {};
      config.url = url;
    }
    return config;
  },
  (error: AxiosError) => {
    // Do something with request error
    console.log(error); // for debug
    Promise.reject(error);
  }
);

// response 拦截器
server.interceptors.response.use(
  (response: AxiosResponse<any>) => {
    if (response.status == 403) {
      ElNotification.error(response.data.msg);
    }
    return response.data;
  },
  (error: AxiosError) => {
    ElNotification.error(error);
    return Promise.reject(error);
  }
);

export { server };
