<script setup lang="ts">
import { useSignUpStore } from '../store';
import { z } from 'zod';

const signUpStore = useSignUpStore();
const { email, resendTimeoutMs, otp } = storeToRefs(signUpStore);
const isLoading = ref(false);

const formSchema = toTypedSchema(z.object({
	otp: z.string().length(6, { message: 'Код должен содержать 6 символов' }),
}));

watchEffect(() => {
	if (otp.value.length >= 6) {
		handleSubmit();
	}
});

async function handleSubmit() {
	isLoading.value = true;
	await signUpStore.makeConfirmOTPRequest();
	isLoading.value = false;
}
</script>

<template>
	<BasicForm
		header="Подтвердите почту"
		with-back-button
		:validation-schema="formSchema"
		@back="signUpStore.backToFirstStep"
		@submit="handleSubmit"
	>
		<Input
			v-model="otp"
			name="otp"
			placeholder="Код подтверждения"
			:max-length="6"
			:disabled="isLoading"
		/>
		<template #subheader>
			Введите 6-ти значный код, отправленный на <span class="email">{{ email }}</span>
		</template>
		<template #actions>
			<ResendTimer
				v-model="resendTimeoutMs"
				:disabled="isLoading"
				@click="signUpStore.makeResendOtpRequest"
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
    color: $text_color;
    }
</style>
