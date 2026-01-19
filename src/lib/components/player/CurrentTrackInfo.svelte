
<script lang="ts">
  import { activeTrack } from '$lib/stores/playerStore';
</script>

<a href="/now-playing" class="track-info-link">
  {#if $activeTrack}
    <div class="cover-art-placeholder">🎵</div>
    <div class="text-info">
      <span class="title">{$activeTrack.title ?? 'Desconocido'}</span>
      <span class="artist">{$activeTrack.artist ?? 'Desconocido'}</span>
    </div>
  {:else}
    <div class="text-info">
      <span>Selecciona una canción</span>
    </div>
  {/if}
</a>

<style>
  .track-info-link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    text-decoration: none;
    color: inherit;
    min-width: 200px; 
  }
  .cover-art-placeholder {
    width: 40px;
    height: 40px;
    background-color: #e0e0e0;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .text-info {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .title, .artist {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .title {
    font-weight: 500;
  }
  .artist {
    font-size: 0.8rem;
    color: #555;
  }
</style>