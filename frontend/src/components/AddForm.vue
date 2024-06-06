<script setup lang="ts">
import { ref } from 'vue';
import { instance } from '../connection/axios';
import { ProductSchemaI } from '../assets/productSchema';
import CustomButton from './CustomButton.vue';
import { encodeURIProductSchema } from '../helpers/encodeURIProductSchema';
import router from '../router';

const productName = ref("");
const price = ref(0);
const quantity = ref(0);

function onSubmit() {
    const form: ProductSchemaI = {
        product_name: productName.value,
        price: price.value,
        quantity: quantity.value
    };

    let uriForm = encodeURIProductSchema(form);

    router.replace("/");

    instance.post("/create", uriForm);


}
</script>
<template>

    <form @submit.prevent="onSubmit()" class="flex flex-col gap-4 overflow-hidden rounded-sm ">
        <header class="bg-primary p-2 text-bold">
            <h1 class="text-2xl text-center text-white font-bold">Add a product</h1>
        </header>
        <div class="p-2 flex flex-col gap-2">
            <label class="flex flex-col gap-2">
                Product Name <input v-model="productName" placeholder="product name"
                    class="dark:bg-orange-500/20 rounded-sm p-2">
            </label>

            <label class="flex flex-col gap-2">
                Price <input v-model="price" placeholder="price" class="dark:bg-orange-500/20 rounded-sm p-2"
                    type="number" step="0.01">
            </label>
            <label class="flex flex-col gap-2">
                Quantity <input v-model="quantity" placeholder="quantity" class="dark:bg-orange-500/20 rounded-sm p-2"
                    type="number" step="1">
            </label>
        </div>
        <div class="bg-primary flex flex-row justify-center p-2">
            <button type="submit" class="w-1/2">
                <CustomButton class="text-xl font-bold text-primary">submit</CustomButton>
            </button>
        </div>
    </form>
</template>