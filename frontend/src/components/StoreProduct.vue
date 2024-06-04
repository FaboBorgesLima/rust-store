<script setup lang="ts">
import { defineProps } from 'vue';
import SectionContainer from './SectionContainer.vue';
defineProps<{ productName: string, price: number, quantity: number }>();
</script>
<template>
    <SectionContainer>
        <h1>{{ productName }}</h1>
        <p>{{ price }}</p>
        <p>{{ quantity }}</p>
    </SectionContainer>
</template>