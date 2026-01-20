<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { position, duration, progress } from '$lib/stores/playbackStore';

  function formatTime(seconds: number): string {
    if (seconds === null || isNaN(seconds)) return '0:00';
    const min = Math.floor(seconds / 60);
    const sec = Math.floor(seconds % 60).toString().padStart(2, '0');
    return `${min}:${sec}`;
  }

  // --- NUEVA LÓGICA DE INTERACCIÓN ---
  let progressBarElement: HTMLProgressElement;

  function handleSeek(event: MouseEvent) {
    if (!$duration || !progressBarElement) return;

    // Obtenemos las dimensiones y la posición de la barra de progreso.
    const rect = progressBarElement.getBoundingClientRect();
    // Calculamos la posición del clic del ratón relativo al inicio de la barra.
    const clickX = event.clientX - rect.left;
    
    // Calculamos el porcentaje (de 0 a 1) donde se hizo clic.
    const percentage = clickX / rect.width;
    
    // Calculamos el nuevo tiempo en segundos.
    const newPosition = $duration * percentage;
    
    console.log(`Seek al ${Math.round(percentage * 100)}% -> ${newPosition.toFixed(2)}s`);
    
    // Actualizamos nuestro store local inmediatamente para una respuesta visual instantánea.
    position.set(newPosition);

    // Llamamos al comando de Rust para que el motor de audio salte a ese tiempo.
    invoke('seek_track', { positionSecs: newPosition });
  }
</script>

<div class="progress-bar-container">
  <span class="time-label current-time">{formatTime($position)}</span>
  
  <!-- Hacemos que el contenedor sea clickeable y le asignamos la referencia -->
  <div class="progress-bar-wrapper" on:click={handleSeek}>
    <progress 
      class="progress-bar"
      bind:this={progressBarElement}
      value={$progress} 
      max="100"
    ></progress>
  </div>

  <span class="time-label total-duration">{formatTime($duration)}</span>
</div>

<style>
  .progress-bar-container {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    max-width: 500px;
  }
  .time-label {
    font-family: monospace;
    font-size: 0.8rem;
    color: #555;
  }
  .progress-bar-wrapper {
    flex-grow: 1;
    cursor: pointer;
    padding: 8px 0; /* Aumenta el área vertical clickeable */
    margin: -8px 0; /* Compensa el padding para no afectar el layout */
  }
  .progress-bar {
    width: 100%;
    height: 8px;
    -webkit-appearance: none;
    appearance: none;
  }
  /* Estilos para Chrome/Safari */
  .progress-bar::-webkit-progress-bar {
    background-color: #eee;
    border-radius: 4px;
  }
  .progress-bar::-webkit-progress-value {
    background-color: #007bff;
    border-radius: 4px;
    transition: width 0.5s linear; /* Transición suave */
  }
</style>