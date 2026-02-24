<script setup lang="ts">
const props = withDefaults(defineProps<{ size?: 'sm' | 'md' | 'lg' }>(), { size: 'md' });
</script>

<template>
	<span :class="[`loader`, props.size]" />
</template>

<style scoped lang="scss">
   .loader {

    border: 5px solid ;
    border-bottom-color: transparent;
    border-radius: 50%;
    display: inline-block;
    box-sizing: border-box;
    animation: rotation 1s linear infinite;
    &.sm {
        border-width: 2px;
        height: 24px;
        width: 24px;
     }
     &.md {
        height: 48px;
        width: 48px;
        border-width: 5px;
     }
        &.lg {
            height: 72px;
            width: 72px;
            border-width: 8px;
        }
    }

    @keyframes rotation {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }

    }
</style>
