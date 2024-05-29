import axios, { AxiosInstance } from "axios";

import.meta.env;

export const instance = function createInstance(
    env: ImportMetaEnv
): AxiosInstance {
    let baseURL = "http://127.0.0.1:8080";

    if (env.PROD) baseURL = "http://api.rust-store.com";

    return axios.create({
        baseURL,
    });
};
