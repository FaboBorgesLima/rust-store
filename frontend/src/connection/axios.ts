import axios, { AxiosInstance } from "axios";

const prod = import.meta.env.PROD;

export const instance = createInstance(prod);
function createInstance(prod: boolean): AxiosInstance {
    let baseURL = "http://127.0.0.1:8080";

    if (prod) baseURL = "http://api.rust-store.com";

    return axios.create({
        baseURL,
    });
}
