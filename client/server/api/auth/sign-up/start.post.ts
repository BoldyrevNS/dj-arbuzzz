import { z } from 'zod';

const bodySchema = z.object({
	email: z.email(),
});

export default defineEventHandler(async (event) => {
	console.log('sign-up-start called');
	const { email } = await readValidatedBody(event, bodySchema.parse);
	const config = useRuntimeConfig(event);

	try {
		const res = await fetch(`${config.public.apiBase}/auth/sign_up_start`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({ email }),
		});

		if (!res.ok) {
			const errorData = await res.json().catch(() => ({}));
			throw createError({
				statusCode: res.status,
				message: errorData.message || 'Failed to start sign up',
			});
		}

		return await res.json();
	}
	catch (error: any) {
		console.log(error);

		if (error.statusCode) {
			throw error;
		}

		throw createError({
			statusCode: 500,
			message: error.message || 'Failed to start sign up',
		});
	}
});
