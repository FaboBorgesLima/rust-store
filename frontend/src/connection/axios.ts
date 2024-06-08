import axios, { AxiosInstance } from "axios";

const prod = import.meta.env.PROD;

export const instance = createInstance(prod);
function createInstance(prod: boolean): AxiosInstance {
    let baseURL = "http://127.0.0.1:8080";

    if (prod) baseURL = "http://rust-store.com:8080";

    return axios.create({
        baseURL,
    });
}
