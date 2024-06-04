<script setup lang="ts">
import SectionContainer from './SectionContainer.vue';
import { instance } from '../connection/axios';
defineProps<{ productName: string, price: number, quantity: number, id: number }>();

function deleteProduct(id: number) {
    instance.delete(`/delete?product_id=${id}`);

}
</script>
<template>
    <SectionContainer>
        <h1>{{ productName }}</h1>
        <p>price: {{ price }}</p>
        <p>quantity: {{ quantity }}</p>
        <p>id: {{ id }}</p>
        <button @click="deleteProduct(id)">Delete</button>
    </SectionContainer>
</template>