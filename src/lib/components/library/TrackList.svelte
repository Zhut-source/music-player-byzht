<!-- Archivo: src/lib/components/library/TrackList.svelte -->
<script lang="ts">
  import type { Track } from '$lib/types';
  // 1. Importamos nuestro nuevo componente de item.
  import TrackListItem from './TrackListItem.svelte';

  // Se queda igual: espera una lista de canciones desde fuera.
  export let tracks: Track[] = [];
</script>

<div class="track-list-container">
  <h2>Mi Biblioteca ({tracks.length} canciones)</h2>
  <ul class="track-list">
    <!-- 2. Por cada 'track', renderizamos un componente 'TrackListItem' -->
    <!--    y le pasamos el objeto 'track' completo como una propiedad. -->
    {#each tracks as track (track.path)}
      <TrackListItem {track} />
    {/each}
  </ul>
</div>

<style>
  /* Los estilos ahora son más simples, ya que la mayoría están en el item */
  .track-list-container {
    padding: 1rem 2rem;
  }
  h2 {
    margin-bottom: 1.5rem;
  }
  .track-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
</style>