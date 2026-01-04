<script lang="ts">
  // Por ahora, no hay lógica. Solo definimos el esqueleto.
  // Más adelante, aquí tendremos variables para saber si está en play o pause.
  let isPlaying = false; 
</script>

<div class="player-controls">
  <button class="control-button">
    <!-- Icono de "Anterior" (usaremos texto por ahora) -->
    ⏮
  </button>

  <button class="control-button play-button">
    <!-- Cambiaremos el icono dependiendo del estado 'isPlaying' -->
    {#if isPlaying}
      <!-- Icono de "Pausa" -->
      ⏸
    {:else}
      <!-- Icono de "Play" -->
      ▶️
    {/if}
  </button>

  <button class="control-button">
    <!-- Icono de "Siguiente" -->
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