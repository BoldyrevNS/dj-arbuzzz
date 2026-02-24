<script setup lang="ts">
import z from 'zod';

type SearchTrackResponse = {
	data: {
		tracks: Array<{
			name: string;
			owner_id: number;
			song_id: number;
		}>;
	};
};

const isLoading = ref(false);
const isPlaying = ref(false);
const searchValue = ref('');
const audioElement = ref<HTMLAudioElement | null>(null);
const searchedTracks = ref<Array<{ name: string; owner_id: number; song_id: number }>>([]);
const searchFormLoading = ref(false);

const { data: trackData } = await useFetch<{ data: { name: string | null } }>('/api/radio/current-track', {
	server: true,
	lazy: false,
});

const currentTrackName = computed(() => trackData.value?.data?.name ?? null);

async function togglePlay() {
	if (isPlaying.value) {
		stopPlaying();
	}
	else {
		await startPlaying();
	}
}

function stopPlaying() {
	if (audioElement.value) {
		audioElement.value.pause();
		audioElement.value.src = '';
		audioElement.value.load();
		audioElement.value = null;
		isPlaying.value = false;
	}
}

async function startPlaying() {
	isLoading.value = true;
	const audio = new Audio('api/radio/stream');
	audio.crossOrigin = 'anonymous';
	audio.onplay = () => {
		isPlaying.value = true;
	};
	audio.onpause = () => {
		isPlaying.value = false;
	};
	audio.oncanplay = () => {
		isLoading.value = false;
	};
	await audio.play();
	audioElement.value = audio;
}

async function searchTrack() {
	searchFormLoading.value = true;
	const response = await $fetch<SearchTrackResponse>(`/api/track/search?track_name=${encodeURIComponent(searchValue.value)}`);
	searchedTracks.value = response.data.tracks;
	searchFormLoading.value = false;
}

function clearSearchTracks() {
	searchedTracks.value = [];
	searchValue.value = '';
}

async function selectTrack(owner_id: number, song_id: number) {
	searchFormLoading.value = true;
	await $fetch(`/api/track/select`, {
		method: 'POST',
		body: { owner_id, song_id },
	});
	clearSearchTracks();
	searchFormLoading.value = false;
}

const searchFormSchema = toTypedSchema(z.object({
	searchValue: z.string().min(1, { message: 'Введите название трека' }),
}));
</script>

<template>
	<div class="layout">
		<div class="items">
			<BasicForm header="Плеер">
				<div
					v-if="currentTrackName"
					class="current-track"
				>
					{{ currentTrackName }}
				</div>
				<Spinner v-if="isLoading" />
				<Button
					v-else

					@click="togglePlay()"
				>
					{{ isPlaying ? 'Stop' : 'Play' }}
				</Button>
			</BasicForm>
			<BasicForm header="Очередь">
				Жопа
			</BasicForm>
		</div>
		<BasicForm
			:header="searchedTracks.length === 0 ? 'Найдите трек' : 'Результаты поиска'"
			:validation-schema="searchFormSchema"
			:with-back-button="searchedTracks.length !== 0"
			@submit="searchTrack"
			@back="clearSearchTracks"
		>
			<Spinner v-if="searchFormLoading" />
			<template v-else-if="searchedTracks.length === 0">
				<Input
					v-model="searchValue"
					name="searchValue"
					placeholder="Введите название трека"
				/>
				<Button stretched>
					Найти
				</Button>
			</template>
			<template v-else>
				<Button
					v-for="track in searchedTracks"
					:key="track.song_id"
					class="trackItem"
					@click="selectTrack(track.owner_id, track.song_id)"
				>
					{{ track.name }}
				</Button>
			</template>
		</BasicForm>
	</div>
</template>

<style lang="scss" scoped>
	.layout {
		display: flex;
		flex-flow: wrap;
		gap: 20px;
	}

	.items {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}
	.trackItem {
		&:hover {
			background-color: $glass_item_background_focused;
			cursor: pointer;
		}
	}
</style>
