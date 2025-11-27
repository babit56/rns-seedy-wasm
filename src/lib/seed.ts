import { id_to_name } from './item-map';

type Shop = {
	potions: { name: string; price: number }[];
	gems: { name: string; price: number }[];
};

export class Seed {
	id: number;
	chests: string[][];
	shops: Shop[];

	constructor(seed: Array<string | number>) {
		this.id = <number>seed[0];
		this.chests = [0, 1, 2, 3, 4, 5].map((i) =>
			(<number[]>seed).slice(i * 5 + 6, i * 5 + 11).map((id) => id_to_name(id))
		);
		this.shops = [0, 1, 2, 3]
			.map((i) => (<number[]>seed).slice(i * 14 + 36, i * 14 + 50))
			.map((shop_thing) => ({
				potions: [0, 1, 2].map((i) => ({
					name: id_to_name(shop_thing[i]),
					price: shop_thing[i + 3]
				})),
				gems: [0, 1, 2, 3].map((i) => ({
					name: id_to_name(shop_thing[2 * i + 6]),
					price: shop_thing[2 * i + 7]
				}))
			}));
	}

	chest(index: number): string[] {
		return this.chests.at(index) ?? [];
	}

	shop(index: number): Shop | undefined {
		return this.shops.at(index);
	}
}
