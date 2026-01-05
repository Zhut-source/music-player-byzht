<!-- Archivo: src/routes/now-playing/+page.svelte -->

<script lang="ts">
  import { activeTrack } from '$lib/stores/playerStore';
  // 1. CAMBIO: Importamos `goto` en lugar de `back`.
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';

</script>

<!-- 2. ENVOLVEMOS nuestro contenido en un `div` con la directiva `transition:slide` -->
<div class="now-playing-container" transition:fly={{ duration: 300, y: 20 }}>
  <!-- 3. AÑADIMOS el botón de retorno -->
  <button class="back-button" on:click={() => goto('/library')}>
    &larr; Volver a la Biblioteca
  </button>

  {#if $activeTrack}
    <div class="cover-art-placeholder">🖼️</div>
    
    <h1>{$activeTrack.title ?? 'Título Desconocido'}</h1>
    <h2>{$activeTrack.artist ?? 'Artista Desconocido'}</h2>
    <p>Del álbum: {$activeTrack.album ?? 'Álbum Desconocido'}</p>
    <code>{$activeTrack.path}</code>
  {:else}
    <h1>No se ha seleccionado ninguna canción.</h1>
  {/if}
</div>

<style>
  .now-playing-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start; /* Alineamos arriba para ver el botón */
    padding: 2rem;
    text-align: center;
    position: relative; /* Necesario para posicionar el botón */
    min-height: 100%;
  }

  .back-button {
    /* Posicionamos el botón en la esquina superior izquierda */
    position: absolute;
    top: 1rem;
    left: 1rem;
    
    /* Estilos del botón */
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

  /* El resto de los estilos se quedan igual */
  .cover-art-placeholder {
    width: 300px;
    height: 300px;
    background-color: #eee;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 5rem;
    margin-top: 3rem; /* Espacio para que no choque con el botón */
    margin-bottom: 2rem;
    border-radius: 8px;
  }
  h1 { margin-bottom: 0.5rem; }
  h2 { margin-bottom: 1rem; color: #555; }
</style>