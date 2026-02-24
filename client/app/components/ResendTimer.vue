<script setup lang="ts">
const emit = defineEmits(['click', 'update:modelValue']);
const props = defineProps<{
	modelValue: number | null;
	disabled?: boolean;
}>();
const formattedTimeout = computed(() => {
	if (props.modelValue) {
		return formatTimeout(props.modelValue);
	}
	return null;
});
const resendIntervalRef = ref<NodeJS.Timeout | null>(null);

watch(
	() => props.modelValue,
	(value, oldValue) => {
		if (oldValue === null && value !== null) {
			startResendTimeout();
		}
	},
);

onBeforeMount(() => {
	if (props.modelValue) {
		startResendTimeout();
	}
});

onUnmounted(() => {
	if (resendIntervalRef.value) {
		clearInterval(resendIntervalRef.value);
	}
});

function startResendTimeout() {
	resendIntervalRef.value = setInterval(() => {
		if (props.modelValue) {
			if (
				Math.floor(props.modelValue / 1000) * 1000 - 1000 <= 0
				&& resendIntervalRef.value
			) {
				clearInterval(resendIntervalRef.value);
				emit('update:modelValue', null);
			}
			else {
				emit('update:modelValue', props.modelValue - 1000);
			}
		}
	}, 1000);
}
</script>

<template>
	<Button
		type="button"
		variant="outlined"
		:disabled="!!props.modelValue || props.disabled"
		@click="emit('click')"
	>
		{{
			props.modelValue
				? `Повторная отправка возможна через ${formattedTimeout}`
				: "Отправить еще раз"
		}}
	</Button>
</template>
