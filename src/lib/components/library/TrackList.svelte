
<script lang="ts">
  import type { Track } from '$lib/types';
  import TrackListItem from './TrackListItem.svelte';

   import { invoke } from '@tauri-apps/api/core';
  import { trackList } from '$lib/stores/playerStore';

  export let tracks: Track[] = [];

  async function handleAddFiles() {
    try {
      // 1. Llamamos al comando que abre el diálogo de selección.
      console.log("Abriendo diálogo para añadir archivos...");
      const selectedPaths = await invoke<string[] | null>('open_add_files_dialog');

      if (selectedPaths && selectedPaths.length > 0) {
        // 2. Si el usuario seleccionó archivos, llamamos al comando para procesarlos.
        console.log(`Enviando ${selectedPaths.length} archivos al backend para procesar...`);
        await invoke('add_tracks_to_library', { paths: selectedPaths });

        // 3. Después de procesar, volvemos a pedir la lista completa a la BD.
        console.log("Actualizando la biblioteca desde la base de datos...");
        const updatedTracks = await invoke<Track[]>('fetch_tracks');
        
        // 4. Actualizamos el store global, lo que hará que la UI se refresque.
        trackList.set(updatedTracks);
        console.log("Biblioteca actualizada.");
      } else {
        console.log("El usuario no seleccionó archivos.");
      }
    } catch (e) {
      console.error("Error durante el proceso de añadir archivos:", e);
      alert("Ocurrió un error al añadir las canciones.");
    }
  }
</script>

<div class="track-list-container">
  <div class="header">
    <h2>Mi Biblioteca ({tracks.length} canciones)</h2>
  </div>
  <ul class="track-list">
    {#each tracks as track (track.path)}
      <TrackListItem {track} />
    {/each}
  </ul>

  <button class="add-button" on:click={handleAddFiles} title="Añadir canciones a la biblioteca">
    +
  </button>
</div>

<style>
  .track-list-container {
    padding: 1rem 2rem;
    position: relative; 
    height: 100%; 
  }
  
  .header {
    margin-bottom: 1.5rem;
  }
  
  .track-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .add-button {
    position: fixed; 
    bottom: 110px; 
    right: 30px;
    
    width: 60px;
    height: 60px;
    border-radius: 50%; 
    background-color: #007bff;
    color: white;
    font-size: 2rem;
    line-height: 60px; 
    text-align: center;
    border: none;
    cursor: pointer;
    box-shadow: 0 4px 8px rgba(0,0,0,0.2);
    transition: all 0.2s ease-in-out;
  }
  .add-button:hover {
    background-color: #0056b3;
    transform: scale(1.05);
  }
</style>