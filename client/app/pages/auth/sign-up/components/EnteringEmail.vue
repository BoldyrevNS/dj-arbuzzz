<script setup lang="ts">
import { useSignUpStore } from '../store';
import { toTypedSchema } from '@vee-validate/zod';
import { storeToRefs } from 'pinia';
import { z } from 'zod';

const signUpStore = useSignUpStore();
const { email } = storeToRefs(signUpStore);
const isLoading = ref(false);

const formSchema = toTypedSchema(z.object({
	email: z.email({ message: 'Некорректный email' }),
}));

async function handleSubmit() {
	isLoading.value = true;
	await signUpStore.makeSignupStartRequest();
	isLoading.value = false;
}
</script>

<template>
	<BasicForm
		header="Регистрация"
		with-back-button
		back-href="sign-in"
		:validation-schema="formSchema"
		@submit="handleSubmit"
	>
		<Input
			v-model="email"
			name="email"
			placeholder="E-mail"
			:disabled="isLoading"
		/>
		<template #subheader>
			Введите E-mail для регистрации. Это позволит вам включать треки и ставить лайки!
		</template>
		<template #actions>
			<Button
				type="submit"
				stretched
				:loading="isLoading"
			>
				Зарегистрироваться
			</Button>
		</template>
	</BasicForm>
</template>
