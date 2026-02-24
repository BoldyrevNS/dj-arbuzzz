<script setup lang="ts">
import { z } from 'zod';
import { useSignUpStore } from '../store';

const signUpStore = useSignUpStore();
const { username, password } = storeToRefs(signUpStore);
const isLoading = ref(false);
const confirmPassword = ref('');

const formSchema = toTypedSchema(z.object({
	username: z.string().min(3, { message: 'Имя пользователя должно содержать минимум 3 символа' }),
	password: z.string().min(8, { message: 'Пароль должен содержать минимум 8 символов' }),
	confirmPassword: z.string(),
}).refine(data => data.password === data.confirmPassword, {
	message: 'Пароли не совпадают', path: ['confirmPassword'] }));

async function handleSubmit() {
	isLoading.value = true;
	await signUpStore.makeSignupEndRequest();
	isLoading.value = false;
}
</script>

<template>
	<BasicForm
		header="Завершение регистрации"
		subheader="Введите имя пользователя и пароль, чтобы завершить регистрацию"
		:validation-schema="formSchema"
		with-back-button
		@submit="handleSubmit"
		@back="signUpStore.backToFirstStep"
	>
		<Input
			v-model="username"
			name="username"
			placeholder="Имя пользователя"
			:disabled="isLoading"
		/>

		<PasswordInput
			v-model="password"
			name="password"
			placeholder="Пароль"
			:disabled="isLoading"
		/>
		<PasswordInput
			v-model="confirmPassword"
			name="confirmPassword"
			placeholder="Повторите пароль"
			:disabled="isLoading"
		/>
		<template #actions>
			<Button
				type="submit"
				stretched
				:loading="isLoading"
			>
				Завершить регистрацию
			</Button>
		</template>
	</BasicForm>
</template>
