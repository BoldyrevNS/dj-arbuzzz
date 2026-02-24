export default defineNuxtPlugin(() => {
	globalThis.$fetch = $fetch.create({
		credentials: 'include',
		async onResponseError({ response }) {
			if (response.status === 401 && !response.url.includes('/api/auth/sign-in')) {
				navigateToAuth();
			}
		},
	});
});

function navigateToAuth() {
	if (import.meta.client) {
		window.location.href = '/auth/sign-in';
	}
}
