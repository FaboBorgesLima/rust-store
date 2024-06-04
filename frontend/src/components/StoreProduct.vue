<script setup lang="ts">
import { defineProps } from 'vue';
defineProps<{ productName: string, price: number, quantity: number }>();
</script>
<template>
    <section>
        <h1>{{ productName }}</h1>
        <p>{{ price }}</p>
        <p>{{ quantity }}</p>
    </section>
</template>