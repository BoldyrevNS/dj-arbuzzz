<script setup lang="ts">
import { z } from 'zod';
import { toTypedSchema } from '@vee-validate/zod';

const email = ref('');
const password = ref('');
const isLoading = ref(false);

const formValidationSchema = toTypedSchema(z.object({
	email: z.email({ message: 'Введите корректный E-mail' }),
	password: z.string('Введите пароль').min(8, { message: 'Пароль должен содержать минимум 8 символов' }),
}));

async function handleSubmit(fields: any) {
	isLoading.value = true;
	await $fetch('/api/auth/sign-in', {
		method: 'POST',
		body: {
			email: fields.email,
			password: fields.password,
		},
	});
	isLoading.value = false;
	navigateTo('/');
};
</script>

<template>
	<BasicForm
		header="Вход"
		:validation-schema="formValidationSchema"
		@submit="handleSubmit"
	>
		<Input
			v-model="email"
			name="email"
			placeholder="E-mail"
		/>
		<PasswordInput
			v-model="password"
			name="password"
			placeholder="Пароль"
		/>
		<!-- <Button
			type="link"
			variant="outlined"
			href="restore"
		>
			Забыли пароль?
		</Button> -->
		<template #actions>
			<Button
				type="submit"
				color="success"
				stretched
			>
				Войти
			</Button>
			<Button
				type="link"
				href="sign-up"
				stretched
			>
				Регистрация
			</Button>
		</template>
	</BasicForm>
</template>
