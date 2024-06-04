<script setup lang="ts">
import { watch, ref, Ref, onMounted } from 'vue';
import { instance } from '../connection/axios';
import StoreProduct from '../components/StoreProduct.vue';

const products: Ref<ProductSchemaI[]> = ref([]);

const productName = ref("");
const price = ref("");
const quantity = ref(0);

interface ProductSchemaI {
    product_name: string,
    price: number,
    quantity: number,
};
onMounted(async () => {
    const res = await instance.get("/all");
    console.log(res.data);
    const body: ProductSchemaI[] = res.data?.products;

    products.value = body
})


function onSubmit() {
    const form: ProductSchemaI = {
        product_name: encodeURIComponent(productName.value),
        price: encodeURIComponent(price.value),
        quantity: encodeURIComponent(quantity.value)
    };

    let uriForm = Object.entries(form).map(([key, val]) => `${key}=${val}`).join("&");

    instance.post("/create", uriForm);
}


</script>

<template>
    <div class="container mx-auto m-2">
        <section class="p-2 rounded shadow-md flex flex-col gap-2">
            <p>Find your products</p>
            <StoreProduct v-for="product in products" :productName="product.product_name" :price="product.price"
                :quantity="product.quantity">
            </StoreProduct>
        </section>
        <form @submit.prevent="onSubmit" class="flex flex-col gap-4 p-2 rounded">
            <input v-model="productName" placeholder="product name" class="dark:bg-orange-500/20 rounded p-2">
            <input v-model="price" placeholder="price" class="dark:bg-orange-500/20 rounded p-2">
            <input v-model="quantity" placeholder="quantity" class="dark:bg-orange-500/20 rounded p-2">
            <button type="submit">submit</button>
        </form>

    </div>

</template>