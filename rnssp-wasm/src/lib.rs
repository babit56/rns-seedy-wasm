use std::iter::once;

pub use rnssp::Run;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Unlocks {
    pub darkbite: bool,
    pub timegem: bool,
    pub youkai: bool,
    pub haunted: bool,
    pub gladiator: bool,
    pub sparkblade: bool,
    pub swiftflight: bool,
    pub sacredflame: bool,
    pub ruins: bool,
    pub lakeshrine: bool,
}

#[wasm_bindgen]
impl Unlocks {
    pub fn new(
        darkbite: bool,
        timegem: bool,
        youkai: bool,
        haunted: bool,
        gladiator: bool,
        sparkblade: bool,
        swiftflight: bool,
        sacredflame: bool,
        ruins: bool,
        lakeshrine: bool,
    ) -> Self {
        Self {
            darkbite,
            timegem,
            youkai,
            haunted,
            gladiator,
            sparkblade,
            swiftflight,
            sacredflame,
            ruins,
            lakeshrine,
        }
    }
}

impl Into<rnssp::types::Unlocks> for &Unlocks {
    fn into(self) -> rnssp::types::Unlocks {
        rnssp::types::Unlocks {
            darkbite: self.darkbite,
            timegem: self.timegem,
            youkai: self.youkai,
            haunted: self.haunted,
            gladiator: self.gladiator,
            sparkblade: self.sparkblade,
            swiftflight: self.swiftflight,
            sacredflame: self.sacredflame,
            ruins: self.ruins,
            lakeshrine: self.lakeshrine,
        }
    }
}

/// Converts run data to a list of (pointers to) js values, both strings and numbers
fn get_output(run: &Run) -> Vec<JsValue> {
    let areas = run
        .area_list
        .into_iter()
        .map(|area_index| rnssp::names::get_area_name(area_index).unwrap())
        .map(|s| JsValue::from_str(s));
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

/// Gets data for one seed from given params, in the same format as `seed_data` in JS code. Panics if `unlocks` arg is badly formatted. No error checking, we die like men
///
/// Unlocks is Copy but when JS creates it, it gets killed by GC after use if passed instead of borrowed -_-
#[wasm_bindgen]
pub fn predict_seed(
    seed: usize,
    players: usize,
    high_difficulty: bool,
    unlocks: &Unlocks,
) -> Vec<JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let mut run = Run::new(seed as u32, players as u8, high_difficulty, unlocks.into());
    run.predict_seed();
    get_output(&run)
}

/// Gets new seed data. Unique seeds must be passed cuz I didn't find a better (efficient) way to get the seeds
/// Outputs data in a similar format to `predict_seed`, but for WASM Reasons(tm), the list cannot use JsValues and therefore cannot have mixed typed elements (str/usize).
/// Areas are therefore outputted as an id instead of as a string
/// Options to work around this include:
/// - Changing this code (might be (much) slower, haven't tested that much)
/// - Changing JS code and the cached `seed_data.json` to use ID's instead of strings for areas
/// - Using the single seed generator multiple times in JS. This seems like the best/easiest option atm, it is equally fast as the below fn on my machine
#[wasm_bindgen]
pub fn new_seed_data(
    unique_seeds: Vec<usize>,
    players: usize,
    high_difficulty: bool,
    unlocks: &Unlocks,
) -> JsValue {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let data: Vec<Vec<usize>> = unique_seeds
        .into_iter()
        .enumerate()
        .map(|(_i, seed)| {
            // log(&format!("Iteration: {}, seed: {}", i, seed));
            let mut run = Run::new(seed as u32, players as u8, high_difficulty, unlocks.into());
            run.predict_seed();

            let areas = run.area_list.into_iter();
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
            once(run.map_seed as usize)
                .chain(areas)
                .chain(items)
                .chain(shops)
                .collect()
        })
        .collect();
    serde_wasm_bindgen::to_value(&data).unwrap()
}
