import chest_data from '$lib/chest-data.json'; // slightly smaller BOYE
import type { SeedData } from '$lib/item-map';

import {
	area_to_name,
	chest_to_color,
	chest_to_color_label,
	id_to_gem,
	id_to_gem_key,
	id_to_name,
	id_to_potion,
	type AreaName
} from './item-map';

type Shop = {
	potions: { name: string; price: number; id: number }[];
	gems: { name: string; price: number; key: string; id: number }[];
};

type Item = {
	id: number;
	name: string;
};

type Chest = {
	label?: string; // For display
	name?: string; // For CSS variable
	colorId?: number;
	spriteId?: number; // For the sprite
	items: Item[];
};

const playerCountToLootMap = {
	1: 3,
	2: 4,
	3: 4,
	4: 5
};

export type SeedTTT = {
	id: number;
	shops: Shop[];
	areas: AreaName[];
	chests: Chest[];
};

// Instantiating this class is currently a pretty slow process and takes
// most of the time when changing pages. To improve this, I will have to run a
// Worker thread in the background to preload seeds, but like I doubt anyone
// cares. Lmao.
export class Seed {
	public readonly id: number;
	private shops: Shop[];
	private areas: AreaName[];
	private chests: Chest[];

	constructor(seedData: SeedData) {
		this.id = seedData[0];
		this.areas = [1, 2, 3, 4, 5].map((i) => <AreaName>seedData[i]);

		const seedChests = (chest_data ?? []).find((chest) => chest[0] === this.id)?.slice(1);
		this.chests = [0, 1, 2, 3, 4, 5].map((i) => ({
			label: chest_to_color_label(seedChests?.at(i) ?? 2),
			name: chest_to_color(seedChests?.at(i) ?? 2),
			colorId: seedChests?.at(i),
			spriteId: (seedChests?.at(i) ?? 2) - 2,
			items: (<number[]>seedData)
				.slice(i * 5 + 6, i * 5 + 11)
				.map((id) => ({ id, name: id_to_name(id) }))
		}));

		this.shops = [0, 1, 2, 3]
			.map((i) => (<number[]>seedData).slice(i * 14 + 36, i * 14 + 50))
			.map((shop_thing) => ({
				potions: [0, 1, 2].map((i) => ({
					name: id_to_potion(shop_thing[i]),
					price: shop_thing[i + 3],
					id: shop_thing[i]
				})),
				gems: [0, 1, 2, 3].map((i) => ({
					name: id_to_gem(shop_thing[2 * i + 6]),
					price: shop_thing[2 * i + 7],
					key: id_to_gem_key(shop_thing[2 * i + 6]),
					id: shop_thing[2 * i + 6]
				}))
			}));
	}

	chest(index: number, playerCount: number | undefined = undefined): Chest | undefined {
		if (playerCount === undefined || !(playerCount in playerCountToLootMap)) return undefined;

		const chest = this.chests.at(index);
		if (!chest) return undefined;

		const alwaysFullChest = index === 0 || index === 1;
		if (playerCount === 4 || alwaysFullChest) return chest;

		// 1-3 players have less items and white chests use the same stock
		const itemCount = playerCountToLootMap[playerCount as keyof typeof playerCountToLootMap];
		if (chest.colorId !== 2) {
			return Object.assign(structuredClone(chest), {
				items: chest.items.slice(0, itemCount)
			});
		}

		// White chests are rolled in order together so seeing 3/5 items means your
		// next 3 will reuse the remaining 2, etc.
		//
		// To find a given white chest's loot given this fact, we can count the
		// prior chests and then filter to the window of values in the list of all
		// white chest loot.
		//
		// First two chests always show 5 so we ignore them for this calculation
		const priorWhiteChestCount = this.chests
			.slice(0, index)
			.filter((c, i) => i !== 0 && i !== 1 && c.colorId === 2).length;
		const whiteChests = this.chests.filter((c, i) => i !== 0 && i !== 1 && c.colorId === 2);
		const whiteChestItems = whiteChests.flatMap((c) => c.items);

		const thisChestItems = whiteChestItems.slice(
			priorWhiteChestCount * itemCount,
			priorWhiteChestCount * itemCount + itemCount
		);

		return Object.assign(structuredClone(chest), {
			items: thisChestItems
		});
	}

	// The trivial ones yay!
	shop(index: number): Shop | undefined {
		return this.shops.at(index);
	}

	areaTitle(index: number): string {
		const areaTitle = this.areas.at(index);
		return area_to_name(areaTitle ?? 'extra_moonlit_prescipice');
	}

	areaName(index: number): AreaName {
		return this.areas.at(index) ?? 'extra_moonlit_prescipice'; // silly error type
	}
}
