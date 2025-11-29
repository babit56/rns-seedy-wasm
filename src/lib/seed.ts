import {
	area_to_name,
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

export class Seed {
	id: number;
	chests: Item[][];
	shops: Shop[];
	areas: string[];

	constructor(seed: Array<string | number>) {
		this.id = <number>seed[0];
		this.areas = [0, 1, 2, 3, 4].map((i) => (<string[]>seed)[i + 1]);
		this.chests = [0, 1, 2, 3, 4, 5].map((i) =>
			(<number[]>seed).slice(i * 5 + 6, i * 5 + 11).map((id) => ({ id, name: id_to_name(id) }))
		);
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

	chest(index: number): Item[] {
		return this.chests.at(index) ?? [];
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
