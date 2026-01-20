
import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Track } from '$lib/types';


export const trackList = writable<Track[]>([]);

export const activeTrack = writable<Track | null>(null);

export const isPlaying = writable<boolean>(false);

export const volume = writable<number>(60);


/**
 * Encuentra y devuelve la siguiente canción en la lista basada en la actual.
 * @param currentTrack - La canción que está sonando ahora.
 * @param trackList - La lista completa de canciones.
 * @returns El objeto de la siguiente canción, o `null` si no se puede encontrar.
 */
export function findNextTrack(currentTrack: Track | null, trackList: Track[]): Track | null {
  if (!currentTrack || trackList.length === 0) {
    return trackList.length > 0 ? trackList[0] : null;
  }

  const currentIndex = trackList.findIndex(t => t.path === currentTrack.path);
  if (currentIndex === -1) {
    return trackList.length > 0 ? trackList[0] : null;
  }

  const nextIndex = (currentIndex + 1) % trackList.length;
  return trackList[nextIndex] ?? null;
}

/**
 * Función auxiliar interna para centralizar la lógica de reproducción.
 * Llama a Rust y actualiza los stores.
 * @param track - La canción a reproducir.
 */
async function playTrack(track: Track) {
  try {
    await invoke('play_track', { path: track.path });
    activeTrack.set(track);
    isPlaying.set(true);
  } catch (e) {
    console.error(`Error al intentar reproducir ${track.path}:`, e);
  }
}

/**
 * Reproduce la siguiente canción en la lista `trackList`.
 * Si llega al final, vuelve al principio (loop).
 */
export async function playNext() {
  const list = get(trackList);
  const current = get(activeTrack);

  if (list.length === 0) return;

  const currentIndex = current ? list.findIndex(t => t.path === current.path) : -1;
  
  const nextIndex = (currentIndex + 1) % list.length;
  const nextTrack = list[nextIndex];

  if (nextTrack) {
    await playTrack(nextTrack);
  }
}

/**
 * Reproduce la canción anterior en la lista `trackList`.
 * Si está en la primera, va a la última (loop).
 */
export async function playPrevious() {
  const list = get(trackList);
  const current = get(activeTrack);

  if (list.length === 0) return;

  const currentIndex = current ? list.findIndex(t => t.path === current.path) : -1;

  if (currentIndex === -1) { 
    if (list[0]) await playTrack(list[0]);
    return;
  }
  
  const prevIndex = (currentIndex - 1 + list.length) % list.length;
  const prevTrack = list[prevIndex];

  if (prevTrack) {
    await playTrack(prevTrack);
  }
}