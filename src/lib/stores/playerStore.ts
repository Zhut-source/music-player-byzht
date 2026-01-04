
import { writable } from 'svelte/store';
import type { Track } from '$lib/types';


export const trackList = writable<Track[]>([]);