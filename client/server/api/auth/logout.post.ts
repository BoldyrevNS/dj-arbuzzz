export default defineEventHandler(async (event) => {
	const config = useRuntimeConfig();
	const sessionCookie = getCookie(event, 'x-authenticated');

	if (sessionCookie) {
		try {
			// Отправляем запрос на backend для удаления сессии
			await fetch(`${config.public.apiBase}/auth/logout`, {
				method: 'POST',
				headers: {
					Cookie: `x-authenticated=${sessionCookie}`,
				},
				credentials: 'include',
			});
		}
		catch (error) {
			console.log('Error during backend logout:', error);
		}
	}

	// Удаляем куку на клиенте
	deleteCookie(event, 'x-authenticated');
	await clearUserSession(event);

	return { success: true };
});
