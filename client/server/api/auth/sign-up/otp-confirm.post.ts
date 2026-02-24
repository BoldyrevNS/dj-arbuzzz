import { z } from 'zod';

const bodySchema = z.object({
	token: z.string(),
	otp: z.string(),
});

export default defineEventHandler(async (event) => {
	const { token, otp } = await readValidatedBody(event, bodySchema.parse);
	const config = useRuntimeConfig();

	try {
		const res = await $fetch(`${config.public.apiBase}/sign-up/verify-otp`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({ token, otp }),
		});

		return res;
	}
	catch (error) {
		console.log(error);
		throw createError({
			statusCode: 500,
			message: 'Failed to confirm OTP',
		});
	}
});
