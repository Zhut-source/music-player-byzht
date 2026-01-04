
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { trackList as trackListStore } from '$lib/stores/playerStore';
  import type { Track } from '$lib/types';
 

  let isLoading = false;
  let errorMessage: string | null = null;
  
  async function openFolderDialog() {
    isLoading = true;
    errorMessage = null;

    try {
      const folderPath = await invoke<string | null>('select_folder');

      if (folderPath) {
        console.log('Solicitando lista de canciones a Rust...');
        const newTracks = await invoke<Track[]>('get_tracks', { folderPath });
        console.log(`Respuesta de Rust: ${newTracks.length} canciones encontradas.`);
        
        trackListStore.set(newTracks);
        
        await goto('/library');

      } else {
        console.log('El usuario canceló la selección.');
        isLoading = false;
      }
    } catch (e) {
      console.error('Error:', e);
      errorMessage = `Error en el backend: ${e}`;
      isLoading = false;
    }
  }
</script>



<main>
    <h1>Reproductor de Música</h1>
    <p>Selecciona una carpeta para empezar a escuchar tu música.</p>
    <button on:click={openFolderDialog} disabled={isLoading}>
        {isLoading ? 'Escaneando...' : 'Seleccionar Carpeta de Música'}
    </button>
    {#if errorMessage}
    <div class="error-box"><p>{errorMessage}</p></div>
    {/if}
</main>


<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding-top: 5rem;
    gap: 1.5rem; 
    font-family: sans-serif;
  }
  
  button {
    font-size: 1.1rem;
    padding: 0.8rem 1.5rem;
    border-radius: 8px;
    border: none;
    background-color: #4a8bf5;
    color: white;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  button:hover {
    background-color: #3a75d4;
  }

  button:disabled {
    background-color: #a0a0a0;
    cursor: not-allowed;
  }
  
  .result-box {
    margin-top: 2rem;
    padding: 1rem;
    background-color: #eeeeee;
    border: 1px solid #dddddd;
    border-radius: 8px;
    max-width: 80%;
    text-align: left;
  }

  code {
    display: block; 
    padding: 0.5rem;
    background-color: #ffffff;
    border-radius: 4px;
    word-break: break-all; 
    margin-top: 0.5rem;
  }
</style>