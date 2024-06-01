<script setup lang="ts">
import { RouteLocationRaw, RouterLink } from 'vue-router';
defineProps<{ to: RouteLocationRaw }>()
</script>

<template>
    <RouterLink :to=to
        class="border-2 bg-orange-50 dark:bg-stone-900 border-orange-200 dark:border-orange-50 border-solid rounded-sm group overflow-visible ">
        <div
            class="p-2 outline outline-2 outline-orange-200 dark:outline-orange-50  group-hover:translate-x-1 group-hover:-translate-y-1 rounded-sm transition-all duration-300 bg-orange-50 dark:bg-stone-900 hover:bg-orange-100 hover:dark:bg-orange-500 ">
            <span class="text-orange-500 dark:text-orange-50 block text-lg font-bold text-center">
                <slot></slot>
            </span>
        </div>
    </RouterLink>
</template>