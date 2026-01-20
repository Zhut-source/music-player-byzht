// Archivo: src/lib/stores/playbackStore.ts
import { writable, derived } from 'svelte/store';

// Este store guarda la duración total de la canción actual en segundos.
export const duration = writable<number>(0);

// Este store guarda la posición actual de la canción en segundos.
export const position = writable<number>(0);

// `derived` crea un store cuyo valor se basa en otros stores.
// Aquí, creamos un store `progress` que siempre será un valor entre 0 y 100.
export const progress = derived(
  [position, duration],
  ([$position, $duration]) => {
    if ($duration === 0) {
      return 0; // Evita la división por cero
    }
    return ($position / $duration) * 100;
  }
);

// Usaremos un temporizador de JavaScript (`setInterval`) para actualizar la posición.
// Necesitamos una referencia a él para poder iniciarlo y detenerlo.
let timer: number | null = null;

/**
 * Inicia el temporizador que actualiza la posición de la canción.
 */
export function startTimer() {
  // Si ya hay un temporizador, lo limpiamos primero.
  if (timer) {
    clearInterval(timer);
  }

  // Creamos un nuevo temporizador que se ejecuta cada segundo.
  timer = setInterval(() => {
    // `update` es una función de los stores de Svelte que nos permite
    // modificar el valor actual.
    position.update(p => p + 1);
  }, 1000);
}

/**
 * Detiene el temporizador.
 */
export function stopTimer() {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
}