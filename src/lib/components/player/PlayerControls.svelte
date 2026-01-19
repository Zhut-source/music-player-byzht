<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { isPlaying, activeTrack } from '$lib/stores/playerStore';
  import { playNext, playPrevious } from '$lib/stores/playerStore';

  async function togglePlayPause() {
    if ($activeTrack) {
      if ($isPlaying) {
        await invoke('pause_track');
        isPlaying.set(false);
      } else {
        await invoke('resume_track');
        isPlaying.set(true);
      }
    }
  }
</script>

<div class="player-controls">
  <button class="control-button" title="Anterior" on:click={playPrevious}
    disabled={!$activeTrack}>
    ⏮
  </button>

  <button 
    class="control-button play-button" 
    on:click={togglePlayPause}
    disabled={!$activeTrack}
    title={$isPlaying ? 'Pausar' : 'Reproducir'}
  >
    {#if $isPlaying}
      ⏸
    {:else}
      ▶️
    {/if}
  </button>

  <button class="control-button" title="Siguiente" on:click={playNext}
    disabled={!$activeTrack}>
    ⏭
  </button>
</div>

<style>
  .player-controls {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1.5rem; 
  }

  .control-button {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1.5rem; 
    color: #333;
    transition: transform 0.1s ease-in-out;
  }

  .control-button:hover {
    color: #000;
  }
  
  .control-button:active {
    transform: scale(0.9);
  }

  .play-button {
    font-size: 2.5rem; 
  }

  .control-button:disabled {
    color: #ccc;
    cursor: not-allowed;
    transform: none;
  }
  .control-button:disabled:hover {
    color: #ccc; 
  }
</style>