export enum RestoreStep {
	INITIAL,
	CONFIRM_OTP,
	SET_NEW_PASSWORD,
}

export type OTPMeta = {
	resend_timeout_ms: number;
	token: string;
};
