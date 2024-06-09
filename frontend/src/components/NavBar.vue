<script setup lang="ts">
import NavButton from './NavButton.vue';
import Icon from './Icon.vue';
</script>

<template>
    <nav class="w-full bg-primary p-2">
        <div class="container mx-auto grid grid-cols-1 lg:grid-cols-2 gap-2">
            <div class="flex justify-center lg:justify-start items-center gap-2">
                <Icon class="h-16"></Icon>
                <h1 class="hidden sm:block bold uppercase text-xl underline">rust store</h1>
            </div>
            <div class="grid gap-2 md:grid-cols-2 my-auto">
                <NavButton to="/">Home</NavButton>
                <NavButton to="/about">About</NavButton>
            </div>
        </div>
    </nav>
</template>