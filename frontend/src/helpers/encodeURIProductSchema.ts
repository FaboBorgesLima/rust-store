import { ProductSchemaI } from "../assets/productSchema";
import { applyIfFloat } from "./applyIfFloat";
import { toDecimal } from "./toDecimal";

export function encodeURIProductSchema(product: ProductSchemaI): string {
    let uri = Object.entries(product)
        .map(
            ([key, val]) =>
                `${key}=${encodeURIComponent(
                    applyIfFloat(val, (n) => toDecimal(n))
                )}`
        )
        .join("&");

    return uri;
}
