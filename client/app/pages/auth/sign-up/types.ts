export enum SignUpStep {
	ENTERING_EMAIL,
	OTP_CONFIRMATION,
	SETTING_SIGN_UP_DATA,
}

export type OTPMeta = {
	resend_timeout_ms: number;
	token: string;
};
