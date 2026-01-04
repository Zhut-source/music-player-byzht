
<script lang="ts">
  
  import { invoke } from '@tauri-apps/api/core';

  type Track = {
    path: string;
    title: string | null;
    artist: string | null;
    album: string | null;
    duration_secs: number | null;
  };

  let selectedFolderPath: string | null = null;
  let isLoading = false;

  let tracks: Track[] = [];

  let errorMessage: string | null = null;

  
  async function openFolderDialog() {
  
    isLoading = true;
    selectedFolderPath = null; 
    tracks = [];

    errorMessage = null;

    try {
      const result = await invoke<string | null>('select_folder');

      if (result) {

        console.log('Carpeta seleccionada:', result);
        selectedFolderPath = result;

        console.log('Solicitando lista de canciones a Rust...');
        isLoading = true;

        const trackList = await invoke<Track[]>('get_tracks', { folderPath: result });

        console.log('Respuesta de Rust:', trackList);
        tracks = trackList;

      } else {
        
        console.log('El usuario canceló la selección.');
      }
    } catch (e) {

      console.error('Error al invocar el comando de Rust:', e);
      errorMessage = `Error en el backend: ${e}`;

    } finally {
      isLoading = false;
    }
  }
</script>



<main>
  <h1>Sound-P</h1>
  <p>Selecciona una carpeta para empezar a escuchar tu música.</p>

  <button on:click={openFolderDialog} disabled={isLoading}>
    {isLoading ? 'Abriendo...' : 'Seleccionar Carpeta de Música'}
  </button>

  {#if errorMessage}
    <div class="error-box">
      <p>{errorMessage}</p>
    </div>
  {/if}

  {#if selectedFolderPath}
    <div class="result-box">
      <p><strong>Ruta de la carpeta de música:</strong></p>
      <code>{selectedFolderPath}</code>
    </div>
  {/if}

  {#if tracks.length > 0}
    <div class="result-box">
      <p><strong>Se encontraron {tracks.length} canciones.</strong></p>
      <p>(Revisa la consola de Rust para ver la lista de archivos escaneados)</p>
    </div>
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
    gap: 1.5rem; /* Espacio entre los elementos */
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
    display: block; /* Para que ocupe su propia línea */
    padding: 0.5rem;
    background-color: #ffffff;
    border-radius: 4px;
    word-break: break-all; /* Evita que rutas largas desborden la caja */
    margin-top: 0.5rem;
  }
</style>