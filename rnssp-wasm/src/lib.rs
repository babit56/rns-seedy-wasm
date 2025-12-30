mod utils;

use std::iter::once;

pub use rnssp::Run;
use rnssp::types::Unlocks;
use wasm_bindgen::prelude::*;

pub const UNIQUE_SEEDS: [usize; 131072] = include!("../unique_seeds.txt");

fn get_output(run: &Run) -> Vec<JsValue> {
    let areas = run
        .area_list
        .iter()
        .map(|&area_index| rnssp::names::get_area_name(area_index).unwrap())
        .map(|name| JsValue::from_str(name));
    let items = run
        .chests
        .iter()
        .map(|chest| chest.as_ref().unwrap().items.clone())
        .flatten()
        .map(|num| JsValue::from_f64(num as f64));
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
        .flatten()
        .map(|num| JsValue::from_f64(num as f64));
    once(JsValue::from_f64(run.map_seed as f64))
        .chain(areas)
        .chain(items)
        .chain(shops)
        .collect()
}

#[wasm_bindgen]
pub fn predict_seed(seed: usize, players: usize, high_difficulty: bool) -> Vec<JsValue> {
    let mut run = Run::new(
        seed as u32,
        players as u8,
        high_difficulty,
        Unlocks::with_none(),
    );
    run.predict_seed();
    // let out: Vec<_> = run
    //     .get_csv_line()
    //     .split(",")
    //     .map(|s| s.to_string())
    //     .collect();
    get_output(&run)
}

#[wasm_bindgen]
pub fn new_seed_data(players: usize, high_difficulty: bool) -> Vec<JsValue> {
    // to_value(&predict_seed(...))
    // UNIQUE_SEEDS
    //     .into_iter()
    //     .map(|seed| {
    //         let data = predict_seed(seed, players, high_difficulty);
    //         serde_wasm_bindgen::to_value(&data).unwrap()
    //     })
    //     .collect()
    todo!();
}
