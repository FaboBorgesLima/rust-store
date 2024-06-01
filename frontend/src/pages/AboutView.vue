<script setup lang="ts">
import SectionContainer from '../components/SectionContainer.vue';
</script>

<template>
    <div class="container mx-auto flex flex-col gap-2 m-2">
        <SectionContainer>
            <h2 class="text-xl font-bold">What this project is about?</h2>
            <p>This application is made for demonstration purposes .</p>
        </SectionContainer>
        <SectionContainer>
            <h2 class="text-xl font-bold">What languages, frameworks and tools this project uses?</h2>
            <p>It uses tailwindCSS and vueJS in the frontend, rust in the backend, mariadb as a database and is
                containerized for development and deployment.</p>
        </SectionContainer>
    </div>
</template>