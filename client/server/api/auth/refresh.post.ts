// С cookie-based авторизацией refresh не требуется
// Backend сам управляет сессией через куку x-authenticated
export default defineEventHandler(async (event) => {
	const config = useRuntimeConfig();
	const sessionCookie = getCookie(event, 'x-authenticated');

	if (!sessionCookie) {
		throw createError({
			statusCode: 401,
			message: 'Not authenticated',
		});
	}

	try {
		// Проверяем валидность сессии на backend
		const response = await fetch(`${config.public.apiBase}/auth/session`, {
			method: 'GET',
			headers: {
				Cookie: `x-authenticated=${sessionCookie}`,
			},
			credentials: 'include',
		});

		if (!response.ok) {
			throw new Error('Session invalid');
		}

		await setUserSession(event, {
			user: {
				authenticated: true,
			},
		});

		return { success: true };
	}
	catch (error) {
		console.log(error);
		deleteCookie(event, 'x-authenticated');
		await clearUserSession(event);

		throw createError({
			statusCode: 401,
			message: 'Session expired',
		});
	}
});
