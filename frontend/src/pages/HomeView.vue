<script setup lang="ts">
import { ref, Ref, onMounted } from 'vue';
import { instance } from '../connection/axios';
import StoreProduct from '../components/StoreProduct.vue';
import { ProductSchemaInDbI } from '../assets/productSchemaInDb';
import SectionContainer from '../components/SectionContainer.vue';

const products: Ref<ProductSchemaInDbI[]> = ref([]);

onMounted(async () => {

    instance.get<{ products: ProductSchemaInDbI[] }>("/all").then((res) => {
        const body = res.data?.products;
        products.value = body;
    }).catch(() => {
        products.value = []
    })
})



</script>

<template>
    <div class="container mx-auto m-2 px-2 sm:px-0 grid grid-cols-1 lg:grid-cols-2 gap-2">
        <SectionContainer class="col-span-2 p-2" v-if="!products">
            <h1 class="text-xl underline">Products not found</h1>
            <p>Add some products so you can view them here</p>
        </SectionContainer>
        <StoreProduct v-for="product in products" :productName="decodeURIComponent(product.product_name)"
            :price="+product.price" :id="product.product_id" :quantity="+product.quantity">
        </StoreProduct>
    </div>
</template>