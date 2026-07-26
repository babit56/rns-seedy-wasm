use std::fs::File;

use rnssp::{
    Run,
    types::{StartingArea, Unlocks},
};

fn get_unique_seeds(starting_area: StartingArea) -> Vec<u32> {
    // NOTE: assumes there are 2^17 unique seeds
    let unique_seeds = 2usize.pow(17);
    let mut states: Vec<(u32, u32)> = Vec::with_capacity(unique_seeds);
    let mut seeds: Vec<u32> = Vec::with_capacity(unique_seeds);
    for seed in 0.. {
        // Check if we have found all seeds
        if states.len() == unique_seeds {
            break;
        }
        // Check if this is a new unique seed
        let short_state = rnssp::get_short_state(seed, starting_area);
        if states.contains(&short_state) {
            continue;
        }
        states.push(short_state);
        seeds.push(seed);
    }
    seeds
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
struct Output<'a> {
    area_options: &'a [Vec<usize>],
    chest_options: &'a [Vec<usize>],
    seeds: &'a [Vec<usize>],
}

fn generate_json(area: StartingArea, seeds: Vec<u32>, filename: &str) {
    let mut data = vec![];
    // let mut area_combos: Vec<Vec<usize>> = vec![];
    // let mut chest_combos: Vec<Vec<usize>> = vec![];
    for seed in seeds {
        let players = 4;
        let high_difficulty = true;
        let unlocks = Unlocks::with_all();
        let mut run = Run::new(seed, players, high_difficulty, area, unlocks);
        run.predict_seed();

        // let area_index = area_combos.len();
        let areas = run.area_list.into_iter().map(|a| a as usize);
        // area_combos.push(areas.collect());

        // let chest_index = chest_combos.len();
        let chest_colors = run
            .chests
            .iter()
            .map(|chest| chest.as_ref().unwrap().color as usize);
        // chest_combos.push(chest_colors.collect());

        let items = run
            .chests
            .iter()
            .map(|chest| chest.as_ref().unwrap().items.clone())
            .flatten();
        let shops = run
            .shops
            .iter()
            .map(|shop| shop.as_ref().unwrap())
            .map(|shop| {
                let potions = shop.potions.iter().map(|pot| pot.potion_id);
                let potion_prices = shop.potions.iter().map(|pot| pot.price);
                let gems = shop
                    .gems
                    .iter()
                    .map(|gem| [gem.gem_id, gem.price])
                    .flatten();
                potions.chain(potion_prices).chain(gems)
            })
            .flatten();
        // let seed_data = [run.map_seed as usize, area_index, chest_index]
        let seed_data = [run.map_seed as usize]
            .into_iter()
            .chain(areas)
            .chain(items)
            .chain(shops)
            .chain(chest_colors)
            .collect::<Vec<_>>();
        data.push(seed_data);
    }

    // let out = Output {
    //     area_options: &area_combos,
    //     chest_options: &chest_combos,
    //     seeds: &data,
    // };

    let file = File::create(filename).unwrap();
    serde_json::to_writer(file, &data).unwrap();
}

// TODO split out item stuff
// Generating takes almost 2min per, idk why its 4x slower than the cli generator
fn main() {
    let areas = [
        (StartingArea::RandomKingdom, false),
        (StartingArea::RandomExtra, false),
        (StartingArea::TrueRandom, false),
        (StartingArea::ChaoticRandom, false),
        (StartingArea::RandomKingdom, true),
        (StartingArea::RandomExtra, true),
        (StartingArea::TrueRandom, true),
        (StartingArea::ChaoticRandom, true),
    ];
    for (area, high_diff) in areas {
        let filename = if high_diff {
            format!("rand-area-{}-highdiff.json", area as usize)
        } else {
            format!("rand-area-{}.json", area as usize)
        };
        println!("Area: {area:?}");
        let seeds = get_unique_seeds(area);
        println!("Finished collecting unique seeds");
        generate_json(area, seeds, &filename);
        println!("Finished simulating runs, wrote to {filename}");
    }
}
