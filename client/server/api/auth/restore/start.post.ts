import { z } from 'zod';

const bodySchema = z.object({
	email: z.string().email(),
});

export default defineEventHandler(async (event) => {
	const { email } = await readValidatedBody(event, bodySchema.parse);
	const config = useRuntimeConfig();

	try {
		const res = await $fetch(`${config.public.apiBase}/auth/restore/start`, {
			method: 'POST',
			body: { email },
		});

		return res;
	}
	catch (error) {
		console.log(error);
		throw createError({
			statusCode: 500,
			message: 'Failed to start password restore',
		});
	}
});
