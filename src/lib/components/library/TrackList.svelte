
<script lang="ts">
  // Importamos el tipo de dato que representará una canción.
  // Más adelante, este tipo será más complejo (artista, álbum, etc.)
  type Track = {
    id: number;
    title: string;
    duration: string;
  };

  // Usamos datos falsos para visualizar la lista.
  // Más adelante, estos datos vendrán de Rust.
  const fakeTracks: Track[] = [
    { id: 1, title: "Bohemian Rhapsody", duration: "5:55" },
    { id: 2, title: "Stairway to Heaven", duration: "8:02" },
    { id: 3, title: "Hotel California", duration: "6:30" },
    { id: 4, title: "Smells Like Teen Spirit", duration: "5:01" },
    { id: 5, title: "Imagine", duration: "3:01" },
  ];
</script>

<div class="track-list-container">
  <h2>Mi Biblioteca</h2>
  <ul class="track-list">
    <!-- Usamos un bloque {#each} de Svelte para recorrer la lista de canciones -->
    {#each fakeTracks as track (track.id)}
      <li class="track-item">
        <span class="track-title">{track.title}</span>
        <span class="track-duration">{track.duration}</span>
      </li>
    {/each}
  </ul>
</div>

<style>
  .track-list-container {
    padding: 2rem;
  }
  h2 {
    margin-bottom: 1.5rem;
  }
  .track-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .track-item {
    display: flex;
    justify-content: space-between;
    padding: 0.8rem 1rem;
    border-bottom: 1px solid #eee;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  .track-item:hover {
    background-color: #f0f0f0;
  }
  .track-title {
    font-weight: 500;
  }
  .track-duration {
    color: #666;
  }
</style>