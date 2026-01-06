// Archivo: src/routes/now-playing/+page.ts
import { browser } from '$app/environment';

export function load({ parent }) {
    if (browser) {
        return {
            track: history.state?.track
        };
    }
    return { track: undefined };
}