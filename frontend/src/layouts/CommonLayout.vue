<script setup lang="ts">
import NavBar from '../components/NavBar.vue';
import CustomFooter from '../components/CustomFooter.vue';
</script>
<template>
    <NavBar></NavBar>
    <main class="grow">
        <slot></slot>
    </main>
    <CustomFooter></CustomFooter>
</template>