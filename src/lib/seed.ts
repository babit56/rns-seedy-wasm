import chest_data from '$lib/chest-data.json'; // slightly smaller BOYE

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

export class Seed {
	id: number;
	shops: Shop[];
	areas: string[];
	chests: Chest[];

	constructor(seed: Array<string | number>) {
		this.id = <number>seed[0];
		this.areas = [0, 1, 2, 3, 4].map((i) => (<string[]>seed)[i + 1]);

		const seedChests = (chest_data ?? []).find((chest) => chest[0] === this.id)?.slice(1);
		this.chests = [0, 1, 2, 3, 4, 5].map((i) => ({
			label: chest_to_color_label(seedChests?.at(i) ?? 2),
			name: chest_to_color(seedChests?.at(i) ?? 2),
			colorId: seedChests?.at(i),
			spriteId: (seedChests?.at(i) ?? 2) - 2,
			items: (<number[]>seed)
				.slice(i * 5 + 6, i * 5 + 11)
				.map((id) => ({ id, name: id_to_name(id) }))
		}));

		this.shops = [0, 1, 2, 3]
			.map((i) => (<number[]>seed).slice(i * 14 + 36, i * 14 + 50))
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

	chest(index: number): Chest | undefined {
		return this.chests.at(index);
	}

	shop(index: number): Shop | undefined {
		return this.shops.at(index);
	}

	areaName(index: number): AreaName {
		return this.areas[index] as AreaName;
	}

	areaTitle(index: number): string {
		return area_to_name(this.areas[index]);
	}
}
