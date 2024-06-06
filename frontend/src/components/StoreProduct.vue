<script setup lang="ts">
import SectionContainer from './SectionContainer.vue';
import { ref } from 'vue';
import StoreProductShow from '../components/StoreProductShow.vue'
import StoreProductUpdate from '../components/StoreProductUpdate.vue'
const props = defineProps<{ productName: string, price: number, quantity: number, id: number }>();

const isDeleted = ref(false);

const productName = ref(props.productName);
const price = ref(props.price);
const quantity = ref(props.quantity);
const isEditMode = ref(false);




</script>
<template>
    <SectionContainer v-if="!isDeleted">
        <header class="w-full bg-primary text-white p-2 font-bold">
            <h1 class=" text-center font-bold text-lg">{{ productName }}</h1>
        </header>
        <StoreProductUpdate v-model:isEditMode="isEditMode" v-model:productName="productName" v-model:price="price"
            v-model:quantity="quantity" :id="id" v-if="isEditMode" class="col-span-6"></StoreProductUpdate>
        <StoreProductShow v-model:isDeleted="isDeleted" v-model:isEditMode="isEditMode" :productName="productName"
            :price="price" :quantity="quantity" :id="id" v-else class="col-span-6"></StoreProductShow>
    </SectionContainer>
</template>