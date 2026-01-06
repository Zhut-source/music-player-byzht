
import { get } from 'svelte/store';
import { trackList } from '$lib/stores/playerStore';
import { error } from '@sveltejs/kit';

export function load({ params }) {
  const decodedPath = decodeURIComponent(params.path);
  const allTracks = get(trackList);
  const foundTrack = allTracks.find(t => t.path === decodedPath);

  if (foundTrack) {
    return {
      track: foundTrack
    };
  }

  throw error(404, 'Canción no encontrada');
}