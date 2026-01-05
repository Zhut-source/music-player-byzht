<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  // 1. IMPORTAMOS los stores que necesitamos.
  import { isPlaying, activeTrack } from '$lib/stores/playerStore';

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
  <button class="control-button" title="Anterior">
    ⏮
  </button>

  <!-- 3. Conectamos el botón al estado y a la función. -->
  <!-- El `title` cambia dinámicamente. -->
  <!-- El botón está deshabilitado si no hay ninguna canción activa. -->
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

  <button class="control-button" title="Siguiente">
    ⏭
  </button>
</div>

<style>
  .player-controls {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1.5rem; /* Espacio entre los botones */
  }

  .control-button {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1.5rem; /* Tamaño de los iconos */
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
    font-size: 2.5rem; /* El botón de play es más grande */
  }
</style>