export default defineNuxtRouteMiddleware(async (to) => {
	const { loggedIn, fetch } = useUserSession();

	const publicPaths = ['/auth/sign-in', '/auth/sign-up', '/auth/restore'];

	const isPublicPath = publicPaths.some(path => to.path.startsWith(path));

	await fetch();

	if (loggedIn.value && isPublicPath) {
		return navigateTo('/');
	}

	if (!loggedIn.value && !isPublicPath) {
		return navigateTo('/auth/sign-in');
	}
});
