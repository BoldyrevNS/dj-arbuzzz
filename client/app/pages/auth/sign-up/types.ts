export enum SignUpStep {
	ENTERING_EMAIL,
	OTP_CONFIRMATION,
	SETTING_SIGN_UP_DATA,
}

export type OTPMeta = {
	timeout_seconds: number;
	token: string;
};
