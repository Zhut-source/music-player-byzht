
<script lang="ts">
  import TrackList from '$lib/components/library/TrackList.svelte';

  import { trackList as trackListStore } from '$lib/stores/playerStore';

</script>

<TrackList tracks={$trackListStore} />