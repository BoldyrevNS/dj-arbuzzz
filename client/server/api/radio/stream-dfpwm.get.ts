export default defineEventHandler(async (event) => {
	const config = useRuntimeConfig();

	try {
		const response = await fetch(`${config.public.apiBase}/radio/stream-dfpwm`, {
			method: 'GET',
			credentials: 'include',
		});

		if (!response.ok) {
			throw createError({
				statusCode: response.status,
				message: 'Failed to fetch DFPWM stream',
			});
		}

		// Проксируем заголовки потока
		setResponseHeaders(event, {
			'Content-Type': 'audio/dfpwm',
			'Cache-Control': 'no-cache, no-store',
			'Transfer-Encoding': 'chunked',
		});

		// Возвращаем поток напрямую без буферизации
		return sendStream(event, response.body as any);
	}
	catch (error) {
		console.error('[stream-dfpwm] Error:', error);

		throw createError({
			statusCode: 500,
			message: 'Failed to fetch DFPWM stream',
		});
	}
});
