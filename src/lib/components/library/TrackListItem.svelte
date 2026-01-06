
<script lang="ts">
  import type { Track } from '$lib/types';
  import { goto } from '$app/navigation';
  import { activeTrack, isPlaying } from '$lib/stores/playerStore';
  import { invoke } from '@tauri-apps/api/core';
  import { get } from 'svelte/store'; 

  export let track: Track;

 
  $: isThisTrackActive = $activeTrack?.path === track.path;
  
  $: isThisTrackPlaying = isThisTrackActive && $isPlaying;

  function formatDuration(seconds: number | null): string {
    if (seconds === null || isNaN(seconds)) return '--:--';
    const min = Math.floor(seconds / 60);
    const sec = Math.floor(seconds % 60).toString().padStart(2, '0');
    return `${min}:${sec}`;
  }

  async function handlePlayClick(event: MouseEvent) {
    event.stopPropagation();
    
    const currentlyPlaying = get(isPlaying);

    if (isThisTrackActive) {
      if (currentlyPlaying) {
        await invoke('pause_track');
        isPlaying.set(false);
      } else {
        await invoke('resume_track');
        isPlaying.set(true);
      }
    } else {
      await invoke('play_track', { path: track.path });
      activeTrack.set(track);
      isPlaying.set(true);
    }
  }

  function handleItemClick() {
  const encodedPath = encodeURIComponent(track.path);
  
  goto(`/track/${encodedPath}`);
}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<li class="track-item" on:click={handleItemClick} class:playing={isThisTrackActive}>
  <div class="play-button-container">
    <!-- 1. Botón de Play para reproducción inmediata -->
    <button class="play-button" on:click={handlePlayClick}>
      {#if isThisTrackPlaying}
        ⏸ 
      {:else}
        ▶️ 
      {/if}
    </button>
  </div>
  <div class="track-info">
    <div class="main-info">
      <span class="track-title">{track.title ?? 'Título Desconocido'}</span>
      <span class="track-artist">{track.artist ?? 'Artista Desconocido'}</span>
      <span class="track-duration">{formatDuration(track.duration_secs)}</span>
    </div>
    <div class="path-info">
      <span class="track-path">{track.path}</span>
    </div>
  </div>
</li>

<style>
  .track-item {
    display: flex; 
    align-items: center;
    gap: 1rem;
    padding: 0.8rem 1rem;
    border-bottom: 1px solid #eee;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  .track-item:hover {
    background-color: #f0f0f0;
  }
  .track-item.playing {
    background-color: #e0f0ff; /* Un azul claro para destacar */
    border-left: 3px solid #007bff; /* Un borde a la izquierda */
  }
  
  .play-button-container {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .play-button {
    background: none;
    border: 1px solid #ccc;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s;
    color: #555;
  }
  .play-button:hover {
    background-color: #e0e0e0;
    border-color: #aaa;
  }
  
  .track-info {
    flex-grow: 1; /* Ocupa el resto del espacio */
    display: flex;
    flex-direction: column; /* Apila la info principal y la ruta */
    overflow: hidden; /* Evita que el texto largo se desborde */
  }

  .main-info {
    display: grid;
    /* Reutilizamos el grid para alinear Título, Artista y Duración */
    grid-template-columns: 2fr 1fr auto;
    gap: 1rem;
    align-items: center;
    width: 100%;
  }

  .track-title { font-weight: 500; }
  .track-artist { color: #555; }
  .track-duration { color: #666; font-family: monospace; }
  
  .path-info {
    margin-top: 0.25rem;
  }

  .track-path {
    font-size: 0.75rem;
    color: #888;
  }

  /* Estilos comunes para evitar desbordamiento de texto */
  .track-title, .track-artist, .track-path {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>