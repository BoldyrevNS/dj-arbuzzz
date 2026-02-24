import { z } from 'zod';

const bodySchema = z.object({
	token: z.string(),
	username: z.string(),
	password: z.string(),
});

export default defineEventHandler(async (event) => {
	const { token, username, password } = await readValidatedBody(event, bodySchema.parse);
	const config = useRuntimeConfig();

	try {
		const res = await $fetch(`${config.public.apiBase}/auth/sign_up_end`, {
			method: 'POST',
			body: { token, username, password },
		});

		return res;
	}
	catch (error) {
		console.log(error);
		throw createError({
			statusCode: 500,
			message: 'Failed to complete sign up',
		});
	}
});
