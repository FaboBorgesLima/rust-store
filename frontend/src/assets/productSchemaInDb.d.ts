import { ProductSchemaI } from "./productSchema";

export interface ProductSchemaInDbI extends ProductSchemaI {
    product_id: number;
}
