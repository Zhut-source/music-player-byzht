<!-- Archivo: src/routes/+layout.svelte -->

<script lang="ts">
  // 1. Importamos nuestro nuevo componente.
  import PlayerControls from '$lib/components/player/PlayerControls.svelte';
</script>

<div class="app-container">
  <main class="main-content">
    <slot />
  </main>

  <footer class="player-bar">
    <!-- 2. Reemplazamos el texto por el componente. -->
    <PlayerControls />
  </footer>
</div>

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
</style>