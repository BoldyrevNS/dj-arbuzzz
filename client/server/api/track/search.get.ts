import { z } from 'zod';
import { createAuthenticatedFetch } from '../../utils/apiClient';

const querySchema = z.object({
	track_name: z.string(),
});

export default defineEventHandler(async (event) => {
	const { track_name } = await getValidatedQuery(event, querySchema.parse);
	const config = useRuntimeConfig();

	try {
		const response = await createAuthenticatedFetch(
			event,
			`${config.public.apiBase}/track/search?track_name=${encodeURIComponent(track_name)}`,
			{ method: 'GET' },
		);
		return response;
	}
	catch (error) {
		console.error('Error fetching track search:', error);
		throw error;
	}
});
