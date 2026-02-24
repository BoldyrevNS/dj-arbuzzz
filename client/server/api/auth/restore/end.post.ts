import { z } from 'zod';

const bodySchema = z.object({
	token: z.string(),
	password: z.string(),
	email: z.string().email(),
});

export default defineEventHandler(async (event) => {
	const { token, password, email } = await readValidatedBody(event, bodySchema.parse);
	const config = useRuntimeConfig();

	try {
		const res = await $fetch(`${config.public.apiBase}/auth/restore/end`, {
			method: 'POST',
			body: { token, password, email },
		});

		return res;
	}
	catch (error) {
		console.log(error);
		throw createError({
			statusCode: 500,
			message: 'Failed to complete password restore',
		});
	}
});
