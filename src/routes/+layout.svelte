
<script lang="ts">
  import PlayerControls from '$lib/components/player/PlayerControls.svelte';
  import CurrentTrackInfo from '$lib/components/player/CurrentTrackInfo.svelte';
  import VolumeControl from '$lib/components/player/VolumeControl.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { trackList, isPlaying } from '$lib/stores/playerStore';
  import type { Track } from '$lib/types';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { listen } from '@tauri-apps/api/event';
  import { position, duration, startTimer, stopTimer } from '$lib/stores/playbackStore';
  import ProgressBar from '$lib/components/player/ProgressBar.svelte';
  import { playNext } from '$lib/stores/playerStore';
  

  let hasCheckedDb = false;
  let unlistenPlaybackStatus: (() => void) | null = null;
  let unlistenPlaybackEnded: (() => void) | null = null;

  onMount(async () => {
    console.log("Layout montado. Obteniendo biblioteca desde la base de datos...");
    try {
      const savedTracks = await invoke<Track[]>('fetch_tracks');
      
      if (savedTracks && savedTracks.length > 0) {
        console.log(`Biblioteca cargada: ${savedTracks.length} canciones encontradas.`);
        trackList.set(savedTracks);

        if ($page.route.id === '/') {
          console.log("Redirigiendo a /library...");
          await goto('/library');
        }
      } else {
        console.log("No se encontró una biblioteca guardada.");
      }
    } catch (e) {
      console.error("Error al obtener la biblioteca:", e);
    } finally {
      hasCheckedDb = true; 
    }

    unlistenPlaybackStatus = await listen('playback-status-update', (event) => {
      console.log('Evento de playback-status-update recibido:', event.payload);
      const status = event.payload as {
        position_secs: number;
        duration_secs: number;
        is_playing: boolean;
      };

      // Actualizamos el store `isPlaying` globalmente.
      isPlaying.set(status.is_playing);

      if (status.is_playing) {
        // Si el evento dice que se está reproduciendo...
        if (status.duration_secs > 0) {
          // Si es un evento de inicio, actualizamos duración y posición.
          duration.set(status.duration_secs);
          position.set(status.position_secs);
        }
        startTimer(); // Iniciamos/reanudamos el temporizador.
      } else {
        // Si el evento dice que está en pausa, detenemos el temporizador.
        stopTimer();
      }
    });

    unlistenPlaybackEnded = await listen('playback-ended', (event) => {
      console.log('Evento de playback-ended recibido. Reproduciendo siguiente canción...');
      
      // Simplemente llamamos a nuestra función `playNext`.
      // Toda la lógica de encontrar la siguiente canción y reproducirla ya está ahí.
      playNext();
    });

  });
  onDestroy(() => {
    // --- MODIFICACIÓN: Nos aseguramos de limpiar AMBAS suscripciones ---
    if (unlistenPlaybackStatus) {
      unlistenPlaybackStatus();
    }
    if (unlistenPlaybackEnded) {
      unlistenPlaybackEnded();
    }
    stopTimer();
  });
</script>

{#if hasCheckedDb}
  <div class="app-container">
    <main class="main-content">
      <slot />
    </main>

    <footer class="player-bar">
      <div class="player-section left">
        <CurrentTrackInfo />
      </div>
      <div class="player-section center">
        <ProgressBar />
        <PlayerControls />
      </div>
      <div class="player-section right">
        <VolumeControl />
      </div>
    </footer>
  </div>
{/if}

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  .main-content {
    flex-grow: 1;
    overflow-y: auto;
  }

  .player-bar {
    display: grid;
    grid-template-columns: 1fr 2fr 1fr; /* Tres columnas: Izquierda, Centro, Derecha */
    align-items: center;
    gap: 2rem;
    height: 90px;
    padding: 0 1rem;
    background-color: #f5f5f5;
    border-top: 1px solid #e0e0e0;
    flex-shrink: 0;
  }
  
</style>