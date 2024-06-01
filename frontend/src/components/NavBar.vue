<script setup lang="ts">
import NavButton from './NavButton.vue';
</script>

<template>
    <nav class="w-full dark:bg-orange-700 bg-orange-500 p-2">
        <div class="container mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2 ">
            <div class="hidden lg:block xl:col-span-2"></div>
            <NavButton to="/">Home</NavButton>
            <NavButton to="/about">About</NavButton>
        </div>
    </nav>
</template>