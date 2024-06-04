<script setup lang="ts">
import { ref } from 'vue';
import { instance } from '../connection/axios';
import { ProductSchemaI } from '../assets/productSchema';
import CustomButton from './CustomButton.vue';
import { encodeURIProductSchema } from '../helpers/encodeURIProductSchema';

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

    instance.post("/create", uriForm);
}

</script>
<template>
    <form @submit.prevent="onSubmit" class="flex flex-col gap-4 p-2 rounded">
        <input v-model="productName" placeholder="product name" class="dark:bg-orange-500/20 rounded p-2">
        <input v-model="price" placeholder="price" class="dark:bg-orange-500/20 rounded p-2">
        <input v-model="quantity" placeholder="quantity" class="dark:bg-orange-500/20 rounded p-2">
        <button type="submit">
            <CustomButton>submit</CustomButton>
        </button>
    </form>
</template>