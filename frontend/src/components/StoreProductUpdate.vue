<script setup lang="ts">
import { reactive } from 'vue';
import { ProductSchemaInDbI } from '../assets/productSchemaInDb';
import { instance } from '../connection/axios';
import { encodeURIProductSchema } from '../helpers/encodeURIProductSchema';

const id = defineModel("id", { required: true, default: 0 });
const productName = defineModel("productName", { required: true, default: "" });
const price = defineModel("price", { required: true, default: 0 });
const quantity = defineModel("quantity", { required: true, default: 0 });
const isEditMode = defineModel("isEditMode", { required: true, default: false });

const product: ProductSchemaInDbI = reactive({
    product_id: id.value,
    product_name: productName.value,
    price: price.value,
    quantity: quantity.value
});
function updateProduct(product: ProductSchemaInDbI) {
    const form = encodeURIProductSchema(product);
    try {
        instance.put("/update", form);

        productName.value = product.product_name;
        price.value = product.price;
        quantity.value = product.quantity;
        isEditMode.value = false;
    }
    catch {

    }
}
</script>
<template>
    <form @submit.prevent="updateProduct(product)" class="flex flex-col gap-2 p-2">
        <label class="w-full flex flex-row gap-1">
            <span class="font-bold uppercase text-sm">product name:</span>
            <input class="flex-grow common-input px-1" v-model="product.product_name">
        </label>
        <label class="w-full flex flex-row gap-1 px-1">
            <span class="font-bold uppercase text-sm">price:</span>
            <input class="flex-grow common-input px-1" v-model="product.price" type="number" step="0.01">
        </label>
        <label class="w-full flex flex-row gap-1">
            <span class="font-bold uppercase text-sm">quantity:</span>
            <input class="flex-grow common-input px-1" v-model="product.quantity" type="number">
        </label>
        <div class="grid grid-cols-2 gap-2 rounded-sm">
            <button type="submit"
                class="group w-full flex flex-row justify-center hover:bg-green-500/10 transition-all duration-300">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                    stroke="currentColor" class="size-6 group-hover:text-green-500 transition-all">
                    <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                </svg>
            </button>
            <button type="button"
                class="group w-full flex flex-row justify-center hover:bg-red-500/10 transition-all duration-300"
                @click="isEditMode = !isEditMode">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                    stroke="currentColor" class="size-6 group-hover:text-red-500 transition-all">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                </svg>
            </button>
        </div>
    </form>
</template>