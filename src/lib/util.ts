import { browser } from '$app/environment';

// URL data stuff
const urlParams = browser ? new URLSearchParams(window.location.search) : undefined;
const urlSeedString = urlParams?.get('seed') ?? null;
export const urlSeed = urlSeedString ? Number(urlSeedString) : null;
export const urlTab = urlParams?.get('tab') ?? null;
