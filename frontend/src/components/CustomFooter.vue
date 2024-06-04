<script setup lang="ts">
import { RouterLink } from 'vue-router';
import CustomButton from '../components/CustomButton.vue';
</script>
<template>
    <footer class="w-full dark:bg-orange-700 bg-orange-500 p-2">
        <div class="container mx-auto flex flex-row justify-end">
            <RouterLink to="/add">
                <CustomButton>
                    <div class="flex flex-row md:gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                            stroke="currentColor" class="size-9 ">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                        </svg>
                        <span class="hidden md:block my-auto md:pr-2 text-xl">add</span>
                    </div>
                </CustomButton>

            </RouterLink>


        </div>

    </footer>
</template>