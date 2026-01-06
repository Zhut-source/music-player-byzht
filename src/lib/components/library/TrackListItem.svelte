
<script lang="ts">
  import type { Track } from '$lib/types';
  import { goto } from '$app/navigation';
  import { activeTrack, isPlaying } from '$lib/stores/playerStore';
  import { invoke } from '@tauri-apps/api/core';
  import { get } from 'svelte/store'; // <-- Importamos `get` para leer stores sin suscripción

  export let track: Track;

  // --- LÓGICA DE ESTADO PARA EL BOTÓN ---
  // `isThisTrackActive` será `true` si la canción de este item es la que está
  // cargada en el reproductor a nivel global.
  $: isThisTrackActive = $activeTrack?.path === track.path;
  
  // `isThisTrackPlaying` será `true` solo si esta es la canción activa Y está sonando.
  $: isThisTrackPlaying = isThisTrackActive && $isPlaying;

  function formatDuration(seconds: number | null): string {
    if (seconds === null || isNaN(seconds)) return '--:--';
    const min = Math.floor(seconds / 60);
    const sec = Math.floor(seconds % 60).toString().padStart(2, '0');
    return `${min}:${sec}`;
  }

  // --- LÓGICA DE `handlePlayClick` COMPLETAMENTE REESCRITA ---
  async function handlePlayClick(event: MouseEvent) {
    event.stopPropagation();
    
    // Usamos `get` para leer el valor actual, ya que `$isPlaying` podría no estar actualizado
    // en el instante exacto del clic dentro de una función async.
    const currentlyPlaying = get(isPlaying);

    if (isThisTrackActive) {
      // Caso 1: Se hizo clic en la canción que ya está activa.
      if (currentlyPlaying) {
        // Si está sonando, la pausamos.
        await invoke('pause_track');
        isPlaying.set(false);
      } else {
        // Si está pausada, la reanudamos.
        await invoke('resume_track');
        isPlaying.set(true);
      }
    } else {
      // Caso 2: Se hizo clic en una canción nueva.
      // La reproducimos desde el principio sin importar el estado anterior.
      await invoke('play_track', { path: track.path });
      activeTrack.set(track);
      isPlaying.set(true);
    }
  }

  function handleItemClick() {
    // La lógica de navegación se queda igual
    activeTrack.set(track);
    goto('/now-playing');
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<li class="track-item" on:click={handleItemClick}>
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