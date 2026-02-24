<script setup lang="ts">
import { z } from 'zod';
import { useRestoreStore } from '../store';

const restoreStore = useRestoreStore();
const { email, resendTimeoutMs, otp } = storeToRefs(restoreStore);
const isLoading = ref(false);

const formSchema = toTypedSchema(z.object({
	otp: z.string().length(6, { message: 'Код должен содержать 6 символов' }),
}));

watchEffect(() => {
	if (otp.value.length >= 6) {
		handleSubmit();
	}
});

async function handleResend() {
	isLoading.value = true;
	await restoreStore.makeResendOtpRequest();
	isLoading.value = false;
}

async function handleSubmit() {
	isLoading.value = true;
	await restoreStore.makeConfirmOTPRequest();
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
		<Input
			v-model="otp"
			name="otp"
			:disabled="isLoading"
			placeholder="Одноразовый код"
			:max-length="6"
		/>
		<template #subheader>
			6-ти значный код для смены пароля был отправлен на <span class="email">{{ email }}</span>
		</template>
		<template #actions>
			<ResendTimer
				v-model="resendTimeoutMs"
				:disabled="isLoading"
				@click="handleResend"
			/>
			<Button
				type="submit"
				stretched
				:loading="isLoading"
			>
				Подтвердить
			</Button>
		</template>
	</BasicForm>
</template>

<style scoped lang="scss">
.email {
    color: $text-color;
}
</style>
