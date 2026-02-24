<script setup lang="ts">
import { z } from 'zod';
import { useRestoreStore } from '../store';

const restoreStore = useRestoreStore();
const { email } = storeToRefs(restoreStore);
const isLoading = ref(false);

const validationSchema = toTypedSchema(z.object({
	email: z.email({ message: 'Некорректный email' }),
}));

async function handleSubmit() {
	isLoading.value = true;
	await restoreStore.makeRestoreStartRequest();
	isLoading.value = false;
}
</script>

<template>
	<BasicForm
		header="Смена пароля"
		with-back-button
		back-href="sign-in"
		:validation-schema="validationSchema"
		@submit="handleSubmit"
	>
		<Input
			v-model="email"
			name="email"
			placeholder="E-mail"
			:disabled="isLoading"
		/>
		<template #subheader>
			Введите почту на которую регистировали свой аккаунт
		</template>

		<template #actions>
			<Button
				type="submit"
				stretched
				:loading="isLoading"
			>
				Отправить код
			</Button>
		</template>
	</BasicForm>
</template>
