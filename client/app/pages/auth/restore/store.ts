import { RestoreStep, type OTPMeta } from './types';
import { defineStore } from 'pinia';
import { $fetch } from 'ofetch';

export const useRestoreStore = defineStore('restore', () => {
	const currentStep = ref(RestoreStep.INITIAL);
	const email = ref('');
	const otp = ref('');
	const resendTimeoutMs = ref(0);
	const token = ref('');
	const password = ref('');

	function backToFirstStep() {
		currentStep.value = RestoreStep.INITIAL;
		token.value = '';
		otp.value = '';
		resendTimeoutMs.value = 0;
		password.value = '';
	}

	function $reset() {
		currentStep.value = RestoreStep.INITIAL;
		email.value = '';
		otp.value = '';
		resendTimeoutMs.value = 0;
		token.value = '';
		password.value = '';
	}

	async function makeRestoreStartRequest() {
		try {
			const res = await $fetch<OTPMeta>('/api/auth/restore/start', {
				method: 'POST',
				body: { email: email.value },
			});
			resendTimeoutMs.value = res.resend_timeout_ms;
			token.value = res.token;
			currentStep.value = RestoreStep.CONFIRM_OTP;
		}
		catch (error) {
			console.error('Ошибка при запуске восстановления:', error);
		}
	}

	async function makeResendOtpRequest() {
		try {
			const res = await $fetch<OTPMeta>('/api/auth/restore/resend-otp', {
				method: 'POST',
				body: { token: token.value },
			});
			resendTimeoutMs.value = res.resend_timeout_ms;
			token.value = res.token;
		}
		catch (error) {
			console.error('Ошибка при повторной отправке OTP:', error);
		}
	}

	async function makeConfirmOTPRequest() {
		try {
			await $fetch('/api/auth/restore/confirm-otp', {
				method: 'POST',
				body: { token: token.value, otp: otp.value },
			});
			currentStep.value = RestoreStep.SET_NEW_PASSWORD;
		}
		catch (error) {
			console.error('Ошибка при подтверждении OTP:', error);
		}
	}

	async function makeEndRestoreRequest() {
		try {
			await $fetch('/api/auth/restore/end', {
				method: 'POST',
				body: { token: token.value, password: password.value, email: email.value },
			});
			navigateTo('/auth/sign-in');
		}
		catch (error) {
			console.error('Ошибка при установке нового пароля:', error);
		}
	}

	return {
		currentStep,
		email,
		resendTimeoutMs,
		otp,
		password,
		makeRestoreStartRequest,
		makeResendOtpRequest,
		makeConfirmOTPRequest,
		makeEndRestoreRequest,
		backToFirstStep,
		$reset,
	};
});
