export type WebSocketMessage =
	| { type: 'current_track'; data: CurrentTrackData }
	| { type: 'playlist'; data: PlaylistData };

export interface CurrentTrackData {
	name: string | null;
}

export interface PlaylistData {
	items: PlaylistItemData[];
}

export interface PlaylistItemData {
	artist: string;
	title: string;
	duration_sec: number;
}
