import { browser } from '$app/environment';
import { Unlocks, StartingArea, int_to_area } from 'rnssp-wasm';

// URL data stuff
const urlParams = browser ? new URLSearchParams(window.location.search) : undefined;
const urlSeedString = urlParams?.get('seed') ?? null;
const urlDiffString = urlParams?.get('difficulty') ?? null;
const urlAreaString = urlParams?.get('area') ?? null;
const urlUnlockString = urlParams?.get('unlocks') ?? null;
export const urlSeed = urlSeedString ? Number(urlSeedString) : null;
export const urlTab = urlParams?.get('tab') ?? null;
export const urlDiff = urlDiffString ? Number(urlDiffString) : null;
export const urlArea = urlAreaString ? int_to_area(urlAreaString) : null;
export const urlUnlocks = urlUnlockString ? Unlocks.from_bitstring(urlUnlockString) : null;

// Loading things
const loadingCharacterList = ['wizard', 'heavyblade', 'dancer', 'assassin', 'sniper'] as const;
type LoadingCharacter = (typeof loadingCharacterList)[number];
type LoadingCharacterData = {
	name: string; // sprite file
	color: string; // flight ring color
	width: number;
	height: number;
};
const loadingCharacterData: Record<LoadingCharacter, LoadingCharacterData> = {
	wizard: {
		name: 'wizard',
		color: '#694ddd',
		width: 500,
		height: 500
	},
	heavyblade: {
		name: 'heavyblade',
		color: '#e873a8',
		width: 625,
		height: 500
	},
	dancer: {
		name: 'dancer',
		color: '#ffe0a1',
		width: 500,
		height: 500
	},
	assassin: {
		name: 'assassin',
		color: '#4c81ff',
		width: 500,
		height: 500
	},
	sniper: {
		name: 'sniper',
		color: '#597bff',
		width: 500,
		height: 500
	}
};

const randomCharacter =
	loadingCharacterList[Math.floor(loadingCharacterList.length * Math.random())];
export const currentLoadingCharacter = loadingCharacterData[randomCharacter];

export function difficulty_to_icon(difficulty: number): string {
	let ret = "";
	switch (difficulty) {
		case 0:
			ret = "Difficulty_Cute.png";
			break;
		case 1:
			ret = "Difficulty_Normal.png";
			break;
		case 2:
			ret = "Difficulty_Hard.png";
			break;
		case 3:
			ret = "Difficulty_Lunar.png";
			break;
		default:
			ret = "ERROR_ICON";
			break;
	}
	return ret;
}

type Settings = {
	difficulty: number;
	starting_area: StartingArea;
	unlocks: Unlocks;
};

export function isSettingsEqual(self: Settings, other: Settings): bool {
	return self.difficulty === other.difficulty
		&& self.starting_area === other.starting_area
		&& self.unlocks.get_bitstring() === other.unlocks.get_bitstring();
}

export function copySettings(to: Settings, from: Settings) {
	to.difficulty = from.difficulty;
	to.starting_area = from.starting_area;
	to.unlocks = from.unlocks.copy();
}

export function settingsHavePrecalc(self: Settings): bool {
	const valid_starts = [
		StartingArea.RandomKingdom,
		StartingArea.RandomExtra,
		StartingArea.TrueRandom,
		StartingArea.ChaoticRandom,
	];
	return self.unlocks.get_bitstring() === Unlocks.full().get_bitstring()
		&& valid_starts.includes(self.starting_area)
}

export function getPrecalcFile(self: Settings): string {
		return `data/rand-area-${self.starting_area}${self.difficulty > 1 ? "-highdiff" : ""}.json`;
}
