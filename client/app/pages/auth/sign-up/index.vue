<script lang="ts" setup>
import { useSignUpStore } from './store';
import { SignUpStep } from './types';

const signUpStore = useSignUpStore();
const { currentStep } = storeToRefs(signUpStore);

onUnmounted(() => {
	signUpStore.$reset();
});
</script>

<template>
	<EnteringEmail v-if="currentStep === SignUpStep.ENTERING_EMAIL" />
	<OTPConfirm v-else-if="currentStep === SignUpStep.OTP_CONFIRMATION" />
	<SettingSignUpData v-else-if="currentStep === SignUpStep.SETTING_SIGN_UP_DATA" />
</template>
