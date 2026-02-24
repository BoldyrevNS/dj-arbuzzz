import { z } from 'zod';

const bodySchema = z.object({
	token: z.string(),
});

export default defineEventHandler(async (event) => {
	const { token } = await readValidatedBody(event, bodySchema.parse);
	const config = useRuntimeConfig();

	try {
		const res = await $fetch(`${config.public.apiBase}/auth/restore/resend_otp`, {
			method: 'POST',
			body: { token },
		});

		return res;
	}
	catch (error) {
		console.log(error);
		throw createError({
			statusCode: 500,
			message: 'Failed to resend OTP',
		});
	}
});
