<script setup lang="ts">
import { RouteLocationRaw, RouterLink } from 'vue-router';
import CustomButton from './CustomButton.vue';
defineProps<{ to: RouteLocationRaw }>()
</script>

<template>
    <RouterLink :to=to>
        <CustomButton>
            <span class="text-primary block text-lg font-bold text-center">
                <slot></slot>
            </span>
        </CustomButton>
    </RouterLink>
</template>