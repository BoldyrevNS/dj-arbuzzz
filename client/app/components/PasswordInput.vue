<script setup lang="ts">
import PasswordVisibleIcon from '~/assets/icons/password-visible-icon.svg';
import PasswordNonVisibleIcon from '~/assets/icons/password-non-visible-icon.svg';

const props = defineProps<{
	placeholder?: string;
	error?: boolean;
	helperText?: string | null;
	name: string;
}>();

const isPasswordVisible = ref(false);
const isFocused = ref(false);

function togglePasswordVisibility() {
	isPasswordVisible.value = !isPasswordVisible.value;
}

function handleFocus() {
	isFocused.value = true;
}

function handleBlur() {
	isFocused.value = false;
}

const model = defineModel<string>();
</script>

<template>
	<Input
		v-model="model"
		:name="props.name"
		:placeholder="props.placeholder"
		:error="props.error"
		:helper-text="props.helperText"
		:type="isPasswordVisible ? 'text' : 'password'"
		@focus="handleFocus"
		@blur="handleBlur"
	>
		<template #endAdornment>
			<IconButton
				:is-focused="isFocused"
				@click="togglePasswordVisibility"
			>
				<PasswordVisibleIcon v-if="isPasswordVisible" />
				<PasswordNonVisibleIcon v-else />
			</IconButton>
		</template>
	</Input>
</template>
