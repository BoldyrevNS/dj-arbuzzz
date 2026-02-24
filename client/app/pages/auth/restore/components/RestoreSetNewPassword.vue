<script setup lang="ts">
import { z } from 'zod';
import { useRestoreStore } from '../store';

const restoreStore = useRestoreStore();
const { password } = storeToRefs(restoreStore);
const repeatPassword = ref('');
const isLoading = ref(false);

const formSchema = toTypedSchema(z.object({
	password: z.string().min(8, { message: 'Пароль должен содержать минимум 8 символов' }),
	repeatPassword: z.string(),
}).refine(data => data.password === data.repeatPassword, {
	message: 'Пароли не совпадают',
	path: ['repeatPassword'],
}));

async function handleSubmit() {
	isLoading.value = true;
	await restoreStore.makeEndRestoreRequest();
	isLoading.value = false;
}
</script>

<template>
	<BasicForm
		header="Смена пароля"
		:validation-schema="formSchema"
		with-back-button
		@submit="handleSubmit"
		@back="restoreStore.backToFirstStep"
	>
		<PasswordInput
			v-model="password"
			placeholder="Новый пароль"
			name="password"
			:disabled="isLoading"
		/>
		<PasswordInput
			v-model="repeatPassword"
			placeholder="Повторите пароль"
			name="repeatPassword"
			:disabled="isLoading"
		/>
		<template #actions>
			<Button
				type="submit"
				stretched
				:loading="isLoading"
			>
				Сменить пароль
			</Button>
		</template>
	</BasicForm>
</template>
