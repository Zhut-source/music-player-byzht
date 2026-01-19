<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { volume } from '$lib/stores/playerStore';

  $: {
    if (typeof $volume === 'number') {
      invoke('set_volume', { volume: $volume / 100 });
    }
  }  

</script>

<div class="volume-control">
  <span class="volume-icon">🔊</span>
  
  <input 
    type="range"
    class="volume-slider"
    min="0"
    max="100"
    bind:value={$volume}
  />

  <input 
    type="number"
    class="volume-input"
    min="0"
    max="100"
    bind:value={$volume}
  />
  <span class="percent-sign">%</span>
</div>

<style>
  .volume-control {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .volume-icon {
    font-size: 1.2rem;
  }
  .volume-slider {
    width: 100px;
    cursor: pointer;
  }
  .volume-input {
    width: 45px;
    text-align: center;
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 0.2rem;
  }
  /* Oculta las flechas del input numérico */
  .volume-input::-webkit-outer-spin-button,
  .volume-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  .volume-input[type=number] {
    -moz-appearance: textfield;
  }
</style>