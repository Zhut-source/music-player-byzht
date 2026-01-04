<!-- Archivo: src/routes/+page.svelte -->

<script lang="ts">
  // Importamos la función 'invoke' desde la API de Tauri.
  // Este es nuestro puente para hablar con el backend de Rust.
  import { invoke } from '@tauri-apps/api/core';

  // Creamos variables para guardar el estado de nuestra página.
  // 'selectedFolderPath' guardará la ruta que nos devuelva Rust.
  let selectedFolderPath: string | null = null;
  // 'isLoading' nos ayudará a deshabilitar el botón mientras esperamos una respuesta.
  let isLoading = false;

  /**
   * Esta función asíncrona se ejecuta cuando el usuario hace clic en el botón.
   */
  async function openFolderDialog() {
    // 1. Ponemos la UI en estado de "cargando".
    isLoading = true;
    selectedFolderPath = null; // Limpiamos el resultado anterior.

    try {
      // 2. Usamos 'invoke' para llamar al comando 'select_folder' que definimos en Rust.
      //    El nombre debe coincidir exactamente.
      //    Especificamos que esperamos recibir un string o null.
      const result = await invoke<string | null>('select_folder');

      // 3. Procesamos la respuesta de Rust.
      if (result) {
        // Si 'result' tiene un valor (no es null), el usuario seleccionó una carpeta.
        console.log('Carpeta seleccionada:', result);
        selectedFolderPath = result;
      } else {
        // Si 'result' es null, el usuario cerró el diálogo sin seleccionar nada.
        console.log('El usuario canceló la selección.');
      }
    } catch (e) {
      // Si ocurre un error durante la llamada (ej. el comando no existe), lo capturamos.
      console.error('Error al llamar al comando select_folder:', e);
      alert('Ocurrió un error al abrir el diálogo.');
    } finally {
      // 4. Sea cual sea el resultado, quitamos el estado de "cargando".
      isLoading = false;
    }
  }
</script>

<!-- Esta es la parte visual (HTML) de nuestra página -->
<main>
  <h1>Reproductor de Música</h1>
  <p>Selecciona una carpeta para empezar a escuchar tu música.</p>

  <!-- El evento 'on:click' llama a nuestra función 'openFolderDialog'.
       La propiedad 'disabled' usa nuestra variable 'isLoading' para
       evitar que el usuario haga clic múltiples veces. -->
  <button on:click={openFolderDialog} disabled={isLoading}>
    {isLoading ? 'Abriendo...' : 'Seleccionar Carpeta de Música'}
  </button>

  <!-- Este bloque solo se mostrará si 'selectedFolderPath' tiene un valor.
       Es una forma de dar feedback visual al usuario. -->
  {#if selectedFolderPath}
    <div class="result-box">
      <p><strong>Ruta de la carpeta de música:</strong></p>
      <code>{selectedFolderPath}</code>
    </div>
  {/if}
</main>

<!-- Estilos CSS para que la página no se vea tan vacía -->
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