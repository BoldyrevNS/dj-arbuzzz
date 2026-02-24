export default defineEventHandler(async () => {
	const config = useRuntimeConfig();
	try {
		const response = await fetch(`${config.public.apiBase}/radio/current-track`, {
			method: 'GET',
		});

		if (!response.ok) {
			throw createError({
				statusCode: response.status,
				message: 'Failed to fetch current track',
			});
		}

		return response.json();
	}
	catch (error) {
		console.log(error);

		throw createError({
			statusCode: 500,
			message: 'Failed to fetch current track',
		});
	}
});
