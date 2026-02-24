export default defineEventHandler(async () => {
	const config = useRuntimeConfig();

	try {
		const response = await fetch(`${config.public.apiBase}/radio/stream`, {
			method: 'GET',
			credentials: 'include',
		});

		if (!response.ok) {
			throw createError({
				statusCode: response.status,
				message: 'Failed to fetch stream URL',
			});
		}

		return response;
	}
	catch (error) {
		console.log(error);

		throw createError({
			statusCode: 500,
			message: 'Failed to fetch stream URL',
		});
	}
});
