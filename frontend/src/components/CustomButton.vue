<script setup lang="ts">
</script>
<template>
    <div
        class="border-2 bg-orange-50 dark:bg-stone-900 border-orange-200 dark:border-orange-50 border-solid rounded-sm group overflow-visible block">
        <div
            class="p-2 outline outline-2 outline-orange-200 dark:outline-orange-50  group-hover:translate-x-1 group-hover:-translate-y-1 rounded-sm transition-all duration-300 bg-orange-50 dark:bg-stone-900 group-hover:bg-orange-100 group-hover:dark:bg-orange-500 ">
            <slot></slot>
        </div>
    </div>

</template>