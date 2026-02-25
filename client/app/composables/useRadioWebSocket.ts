import type { PlaylistData, WebSocketMessage } from '~/shared/websocket-types';

export function useRadioWebSocket() {
	const currentTrack = ref<string | null>(null);
	const playlist = ref<PlaylistData['items']>([]);
	const isConnected = ref(false);
	const config = useRuntimeConfig();

	let ws: WebSocket | null = null;
	let reconnectTimeout: ReturnType<typeof setTimeout> | null = null;

	const connect = () => {
		const wsUrl = config.public.wsBase;
		console.log('[WebSocket] Connecting to:', wsUrl);

		try {
			ws = new WebSocket(wsUrl);

			ws.onopen = () => {
				console.log('[WebSocket] Connected');
				isConnected.value = true;
			};

			ws.onmessage = (event) => {
				try {
					const message: WebSocketMessage = JSON.parse(event.data);

					if (message.type === 'current_track') {
						currentTrack.value = message.data.name;
					}
					else if (message.type === 'playlist') {
						playlist.value = message.data.items;
					}
				}
				catch (error) {
					console.error('[WebSocket] Parse error:', error);
				}
			};

			ws.onerror = (error) => {
				console.error('[WebSocket] Error:', error);
			};

			ws.onclose = () => {
				console.log('[WebSocket] Disconnected');
				isConnected.value = false;
				ws = null;

				// Попробуем переподключиться через 3 секунды
				reconnectTimeout = setTimeout(() => {
					console.log('[WebSocket] Reconnecting...');
					connect();
				}, 3000);
			};
		}
		catch (error) {
			console.error('[WebSocket] Connection error:', error);
		}
	};

	const disconnect = () => {
		if (reconnectTimeout) {
			clearTimeout(reconnectTimeout);
			reconnectTimeout = null;
		}

		if (ws) {
			ws.close();
			ws = null;
		}
	};

	// Подключаемся при создании
	onMounted(() => {
		connect();
	});

	// Отключаемся при размонтировании
	onBeforeUnmount(() => {
		disconnect();
	});

	return {
		currentTrack: readonly(currentTrack),
		playlist: readonly(playlist),
		isConnected: readonly(isConnected),
		connect,
		disconnect,
	};
}
