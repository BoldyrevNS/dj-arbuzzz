<script setup lang="ts">
import type { NuxtError } from '#app';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const props = defineProps({
	error: Object as () => NuxtError,
});

const handleError = () => clearError({ redirect: '/' });
</script>

<template>
	<div
		class="page"
	>
		<img
			src="/cat.gif"
			alt=""
		>
		<h2>Мда уж: {{ error?.message }}</h2>
		<Button @click="handleError">
			Я хочу уйти отсюда
		</Button>
	</div>
</template>

<style lang="scss" scoped>
    .page {
        background-color: #060402;
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
</style>
