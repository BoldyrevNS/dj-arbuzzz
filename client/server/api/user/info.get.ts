import { createAuthenticatedFetch } from '../../utils/apiClient';

export default defineEventHandler(async (event) => {
	const config = useRuntimeConfig();

	try {
		const data = await createAuthenticatedFetch(
			event,
			`${config.public.apiBase}/user/info`,
			{
				method: 'GET',
			},
		);
		console.log(data, 'user info fetched successfully');
		return data;
	}
	catch (error) {
		console.log(error);
		throw error;
	}
});
