import type { H3Event } from 'h3';

interface FetchOptions extends RequestInit {
	headers?: HeadersInit;
	method?: string;
	body?: any;
}

export async function createAuthenticatedFetch(event: H3Event, url: string, options: FetchOptions = {}) {
	const sessionCookie = getCookie(event, 'x-authenticated');

	if (!sessionCookie) {
		throw createError({
			statusCode: 401,
			message: 'Not authenticated',
		});
	}

	const headers = new Headers(options.headers);
	headers.set('Cookie', `x-authenticated=${sessionCookie}`);
	headers.set('Content-Type', 'application/json');

	const fetchOptions: FetchOptions = {
		...options,
		headers,
		credentials: 'include',
	};

	const response = await fetch(url, fetchOptions);

	if (!response.ok) {
		if (response.status === 401) {
			// Сессия истекла, пытаемся обновить
			try {
				await $fetch('/api/auth/refresh', {
					method: 'POST',
				});

				// Повторяем запрос
				const sessionCookieRetry = getCookie(event, 'x-authenticated');
				if (sessionCookieRetry) {
					headers.set('Cookie', `x-authenticated=${sessionCookieRetry}`);
					const retryResponse = await fetch(url, {
						...options,
						headers,
						credentials: 'include',
					});
					return await retryResponse.json();
				}
			}
			catch (refreshError) {
				deleteCookie(event, 'x-authenticated');
				await clearUserSession(event);
				throw refreshError;
			}
		}
		throw createError({
			statusCode: response.status,
			message: await response.text(),
		});
	}

	return await response.json();
}
