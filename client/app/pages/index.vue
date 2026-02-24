<script lang="ts" setup>
async function handleClick() {
	await $fetch('/api/auth/logout', {
		method: 'POST',
	});
	await navigateTo('/auth/sign-in');
}
</script>

<template>
	<PlayerWidget />
</template>
