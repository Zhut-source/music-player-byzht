
<script lang="ts">
  import { page } from '$app/stores';
  import { activeTrack, isPlaying } from '$lib/stores/playerStore';
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/core';
  
  $: viewedTrack = $page.data.track;

  // La lógica para el estado del botón es la misma y ahora funcionará perfectamente.
  $: isThisTrackActive = $activeTrack?.path === viewedTrack?.path;
  $: isThisTrackPlaying = isThisTrackActive && $isPlaying;

  async function handlePlayPauseClick() {
    if (!viewedTrack) return;
    // ... (la lógica de play/pause se queda igual que en la versión funcional anterior)
    if (isThisTrackPlaying) {
      await invoke('pause_track');
      isPlaying.set(false);
    } else if (isThisTrackActive && !$isPlaying) {
      await invoke('resume_track');
      isPlaying.set(true);
    } else {
      await invoke('play_track', { path: viewedTrack.path });
      activeTrack.set(viewedTrack);
      isPlaying.set(true);
    }
  }
</script>

<div class="track-details-container" transition:fly={{ duration: 250, y: 15 }}>
  <button class="back-button" on:click={() => goto('/library')}>
    &larr; Volver a la Biblioteca
  </button>

  {#if viewedTrack}
    <div class="cover-art-placeholder">🖼️</div>
    <h1>{viewedTrack.title ?? 'Título Desconocido'}</h1>
    <h2>{viewedTrack.artist ?? 'Artista Desconocido'}</h2>
    <button class="main-play-button" on:click={handlePlayPauseClick}>
      {isThisTrackPlaying ? 'Pausar' : 'Reproducir'}
    </button>
  {:else}
    <h1>Canción no encontrada.</h1>
  {/if}
</div>

<style>
  .track-details-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start; 
    padding: 2rem;
    text-align: center;
    position: relative; 
    min-height: 100%;
  }

  .back-button {
    position: absolute;
    top: 1rem;
    left: 1rem;
    
    background: #f0f0f0;
    border: 1px solid #ddd;
    border-radius: 20px;
    padding: 0.5rem 1rem;
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.2s;
  }
  .back-button:hover {
    background: #e0e0e0;
    border-color: #ccc;
  }

  .cover-art-placeholder {
    width: 300px;
    height: 300px;
    background-color: #eee;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 5rem;
    margin-top: 3rem; 
    margin-bottom: 2rem;
    border-radius: 8px;
  }
  h1 { margin-bottom: 0.5rem; }
  h2 { margin-bottom: 1rem; color: #555; }
</style>