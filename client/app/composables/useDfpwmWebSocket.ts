/**
 * Composable для подключения к WebSocket стриму DFPWM аудио
 * Получает бинарные данные в формате DFPWM и управляет подключением
 */
export function useDfpwmWebSocket() {
	const isConnected = ref(false);
	const isPlaying = ref(false);
	const config = useRuntimeConfig();

	let ws: WebSocket | null = null;
	let reconnectTimeout: ReturnType<typeof setTimeout> | null = null;

	// Коллбэк для обработки полученных бинарных данных
	const onDataReceived = ref<((data: ArrayBuffer) => void) | null>(null);

	const connect = () => {
		// Формируем WebSocket URL для DFPWM стрима
		const wsUrl = config.public.wsDfpwm;
		console.log('[DFPWM WebSocket] Connecting to:', wsUrl);

		try {
			ws = new WebSocket(wsUrl);
			ws.binaryType = 'arraybuffer';

			ws.onopen = () => {
				console.log('[DFPWM WebSocket] Connected');
				isConnected.value = true;
			};

			ws.onmessage = (event) => {
				// Получаем бинарные данные DFPWM
				if (event.data instanceof ArrayBuffer) {
					console.log('[DFPWM WebSocket] Received chunk:', event.data.byteLength, 'bytes');

					// Вызываем коллбэк если установлен
					if (onDataReceived.value) {
						onDataReceived.value(event.data);
					}
				}
			};

			ws.onerror = (error) => {
				console.error('[DFPWM WebSocket] Error:', error);
			};

			ws.onclose = () => {
				console.log('[DFPWM WebSocket] Disconnected');
				isConnected.value = false;
				ws = null;

				// Попробуем переподключиться через 3 секунды
				if (isPlaying.value) {
					reconnectTimeout = setTimeout(() => {
						console.log('[DFPWM WebSocket] Reconnecting...');
						connect();
					}, 3000);
				}
			};
		}
		catch (error) {
			console.error('[DFPWM WebSocket] Connection error:', error);
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

		isPlaying.value = false;
	};

	const play = () => {
		isPlaying.value = true;
		if (!ws || ws.readyState !== WebSocket.OPEN) {
			connect();
		}
	};

	const stop = () => {
		isPlaying.value = false;
		disconnect();
	};

	// Устанавливаем обработчик данных
	const setDataHandler = (handler: (data: ArrayBuffer) => void) => {
		onDataReceived.value = handler;
	};

	// Отключаемся при размонтировании
	onBeforeUnmount(() => {
		disconnect();
	});

	return {
		isConnected: readonly(isConnected),
		isPlaying: readonly(isPlaying),
		connect,
		disconnect,
		play,
		stop,
		setDataHandler,
	};
}
