<script setup lang="ts">
import { ref, Ref, onMounted } from 'vue';
import { instance } from '../connection/axios';
import StoreProduct from '../components/StoreProduct.vue';
import { ProductSchemaI } from '../assets/productSchema';

const products: Ref<ProductSchemaI[]> = ref([]);

onMounted(async () => {

    instance.get<{ products: ProductSchemaI[] }>("/all").then((res) => {
        const body: ProductSchemaI[] = res.data?.products;
        products.value = body;
    }).catch(() => {
        products.value = []
    })
})



</script>

<template>
    <div class="container mx-auto m-2 flex flex-col gap-2">
        <StoreProduct v-for="product in products" :productName="product.product_name" :price="+product.price"
            :quantity="+product.quantity">
        </StoreProduct>
    </div>
</template>