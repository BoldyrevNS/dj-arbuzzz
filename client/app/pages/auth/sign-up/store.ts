import { SignUpStep, type OTPMeta } from './types';
import { defineStore } from 'pinia';
import { $fetch } from 'ofetch';

export const useSignUpStore = defineStore('signUp', () => {
	const currentStep = ref<SignUpStep>(SignUpStep.ENTERING_EMAIL);
	const email = ref('');
	const sessionToken = ref('');
	const resendTimeoutMs = ref(0);
	const otp = ref('');
	const username = ref('');
	const password = ref('');

	function backToFirstStep() {
		currentStep.value = SignUpStep.ENTERING_EMAIL;
		sessionToken.value = '';
		otp.value = '';
		username.value = '';
		password.value = '';
	}

	function $reset() {
		currentStep.value = SignUpStep.ENTERING_EMAIL;
		sessionToken.value = '';
		otp.value = '';
		username.value = '';
		password.value = '';
		email.value = '';
	}

	async function makeSignupStartRequest() {
		try {
			const res = await $fetch<OTPMeta>('/api/auth/sign-up/start', {
				method: 'POST',
				body: { email: email.value },
			});
			sessionToken.value = res.token;
			resendTimeoutMs.value = res.resend_timeout_ms;
			currentStep.value = SignUpStep.OTP_CONFIRMATION;
		}
		catch (error) {
			console.error('Ошибка при запуске регистрации:', error);
		}
	}

	async function makeResendOtpRequest() {
		try {
			const res = await $fetch<OTPMeta>('/api/auth/sign-up/resend-otp', {
				method: 'POST',
				body: { token: sessionToken.value },
			});
			resendTimeoutMs.value = res.resend_timeout_ms;
		}
		catch (error) {
			console.error('Ошибка при повторной отправке OTP:', error);
		}
	}

	async function makeConfirmOTPRequest() {
		try {
			await $fetch('/api/auth/sign-up/otp-confirm', {
				method: 'POST',
				body: { token: sessionToken.value, otp: otp.value },
			});
			currentStep.value = SignUpStep.SETTING_SIGN_UP_DATA;
		}
		catch (error) {
			console.error('Ошибка при подтверждении OTP:', error);
		}
	}

	async function makeSignupEndRequest() {
		try {
			await $fetch('/api/auth/sign-up/end', {
				method: 'POST',
				body: {
					token: sessionToken.value,
					username: username.value,
					password: password.value,
				},
			});
			navigateTo('/auth/sign-in');
		}
		catch (error) {
			console.error('Ошибка при завершении регистрации:', error);
		}
	}

	return {
		email,
		resendTimeoutMs,
		currentStep,
		otp,
		username,
		password,
		makeSignupStartRequest,
		makeResendOtpRequest,
		makeConfirmOTPRequest,
		makeSignupEndRequest,
		backToFirstStep,
		$reset,
	};
});
