
<script lang="ts">
  import type { Track } from '$lib/types';

  export let tracks: Track[] = [];

  function formatDuration(seconds: number | null): string {
    if (seconds === null || isNaN(seconds)) {
      return '--:--';
    }
    const min = Math.floor(seconds / 60);
    const sec = Math.floor(seconds % 60).toString().padStart(2, '0');
    return `${min}:${sec}`;
  }
</script>

<div class="track-list-container">
  <h2>Mi Biblioteca ({tracks.length} canciones)</h2>
  <ul class="track-list">
    {#each tracks as track (track.path)}
      <li class="track-item">
        <span class="track-title">{track.title ?? 'Título Desconocido'}</span>
        <span class="track-artist">{track.artist ?? 'Artista Desconocido'}</span>
        <span class="track-duration">{formatDuration(track.duration_secs)}</span>
      </li>
    {/each}
  </ul>
</div>

<style>
  .track-list-container { padding: 1rem 2rem; }
  h2 { margin-bottom: 1.5rem; }
  .track-list { list-style: none; padding: 0; margin: 0; }
  .track-item {
    display: grid;
    grid-template-columns: 2fr 1fr auto;
    gap: 1rem;
    align-items: center;
    padding: 0.8rem 1rem;
    border-bottom: 1px solid #eee;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  .track-item:hover { background-color: #f0f0f0; }
  .track-title { font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .track-artist { color: #555; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .track-duration { color: #666; font-family: monospace; }
</style>