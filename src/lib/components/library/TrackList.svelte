
<script lang="ts">
  import type { Track } from '$lib/types';
  import TrackListItem from './TrackListItem.svelte';

  export let tracks: Track[] = [];
</script>

<div class="track-list-container">
  <h2>Mi Biblioteca ({tracks.length} canciones)</h2>
  <ul class="track-list">
    {#each tracks as track (track.path)}
      <TrackListItem {track} />
    {/each}
  </ul>
</div>

<style>
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