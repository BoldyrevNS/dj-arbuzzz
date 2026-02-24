import { z } from 'zod';
import { createAuthenticatedFetch } from '../../utils/apiClient';

const bodySchema = z.object({
	owner_id: z.number(),
	song_id: z.number(),
});

export default defineEventHandler(async (event) => {
	const { owner_id, song_id } = await readValidatedBody(event, bodySchema.parse);
	const config = useRuntimeConfig();

	try {
		const response = await createAuthenticatedFetch(
			event,
			`${config.public.apiBase}/track/select`,
			{ method: 'POST', body: JSON.stringify({ owner_id, song_id }) },
		);
		return response;
	}
	catch (error) {
		console.error('Error selecting track:', error);
		throw error;
	}
});
