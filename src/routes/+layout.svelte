
<script lang="ts">
  import PlayerControls from '$lib/components/player/PlayerControls.svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { trackList } from '$lib/stores/playerStore';
  import type { Track } from '$lib/types';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  let hasCheckedDb = false;

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
  });
</script>

{#if hasCheckedDb}
  <div class="app-container">
    <main class="main-content">
      <slot />
    </main>

<a href="/now-playing" class="player-bar-link">
    <footer class="player-bar">
      <PlayerControls />
    </footer>
  </a>
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
    flex-shrink: 0;
    background-color: #f5f5f5;
    border-top: 1px solid #e0e0e0;
    padding: 0.75rem 1rem; /* Ajustamos el padding */
    display: flex; /* Centramos el contenido */
    justify-content: center;
    align-items: center;
  }
  .player-bar-link {
    text-decoration: none;
    color: inherit;
    display: block; /* Hacemos que ocupe todo el ancho */
  }
</style>