<script setup lang="ts">
import { useRestoreStore } from './store';
import { RestoreStep } from './types';

const restoreStore = useRestoreStore();

const { currentStep } = storeToRefs(restoreStore);

onUnmounted(() => {
	restoreStore.$reset();
});
</script>

<template>
	<RestoreInitial v-if="currentStep === RestoreStep.INITIAL" />
	<RestoreOTPConfirm v-else-if="currentStep === RestoreStep.CONFIRM_OTP" />
	<RestoreSetNewPassword v-else-if="currentStep === RestoreStep.SET_NEW_PASSWORD" />
</template>
