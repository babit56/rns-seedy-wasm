use std::iter::once;

pub use rnssp::Run;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub glacier: bool,
    pub memory: bool,
    pub cultist: bool,
    pub painters: bool,
    pub daynight: bool,
    pub sharpedge: bool,
    pub oceans: bool,
    pub performers: bool,
    pub miners: bool,
    pub teaparty: bool,
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
        glacier: bool,
        memory: bool,
        cultist: bool,
        painters: bool,
        daynight: bool,
        sharpedge: bool,
        oceans: bool,
        performers: bool,
        miners: bool,
        teaparty: bool,
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
            glacier,
            memory,
            cultist,
            painters,
            daynight,
            sharpedge,
            oceans,
            performers,
            miners,
            teaparty,
        }
    }

    pub fn full() -> Self {
        Self {
            darkbite: true,
            timegem: true,
            youkai: true,
            haunted: true,
            gladiator: true,
            sparkblade: true,
            swiftflight: true,
            sacredflame: true,
            ruins: true,
            lakeshrine: true,
            glacier: true,
            memory: true,
            cultist: true,
            painters: true,
            daynight: true,
            sharpedge: true,
            oceans: true,
            performers: true,
            miners: true,
            teaparty: true,
        }
    }

    pub fn toggle_index(&mut self, i: usize) {
        match i {
            0 => self.darkbite = !self.darkbite,
            1 => self.timegem = !self.timegem,
            2 => self.youkai = !self.youkai,
            3 => self.haunted = !self.haunted,
            4 => self.gladiator = !self.gladiator,
            5 => self.sparkblade = !self.sparkblade,
            6 => self.swiftflight = !self.swiftflight,
            7 => self.sacredflame = !self.sacredflame,
            8 => self.ruins = !self.ruins,
            9 => self.lakeshrine = !self.lakeshrine,
            10 => self.glacier = !self.glacier,
            11 => self.memory = !self.memory,
            12 => self.cultist = !self.cultist,
            13 => self.painters = !self.painters,
            14 => self.daynight = !self.daynight,
            15 => self.sharpedge = !self.sharpedge,
            16 => self.oceans = !self.oceans,
            17 => self.performers = !self.performers,
            18 => self.miners = !self.miners,
            19 => self.teaparty = !self.teaparty,
            _ => (),
        };
    }

    pub fn check_index(&self, i: usize) -> bool {
        match i {
            0 => self.darkbite,
            1 => self.timegem,
            2 => self.youkai,
            3 => self.haunted,
            4 => self.gladiator,
            5 => self.sparkblade,
            6 => self.swiftflight,
            7 => self.sacredflame,
            8 => self.ruins,
            9 => self.lakeshrine,
            10 => self.glacier,
            11 => self.memory,
            12 => self.cultist,
            13 => self.painters,
            14 => self.daynight,
            15 => self.sharpedge,
            16 => self.oceans,
            17 => self.performers,
            18 => self.miners,
            19 => self.teaparty,
            _ => true,
        }
    }

    pub fn from_bitstring(s: &str) -> Option<Self> {
        let bits = usize::from_str_radix(s, 2).ok()?;
        Some(rnssp::types::Unlocks::from_bitstring(bits).into())
    }

    pub fn get_bitstring(&self) -> String {
        format!(
            "{:b}",
            Into::<rnssp::types::Unlocks>::into(self).get_bitstring()
        )
    }

    pub fn is_full(&self) -> bool {
        self == &Unlocks::full()
    }

    /// Create a new equal struct. JS cannot use the normal .clone() or copy semantics
    pub fn copy(&self) -> Self {
        self.clone()
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
            glacier: self.glacier,
            memory: self.memory,
            cultist: self.cultist,
            painters: self.painters,
            daynight: self.daynight,
            sharpedge: self.sharpedge,
            oceans: self.oceans,
            performers: self.performers,
            miners: self.miners,
            teaparty: self.teaparty,
        }
    }
}

impl From<rnssp::types::Unlocks> for Unlocks {
    fn from(other: rnssp::types::Unlocks) -> Self {
        Self {
            darkbite: other.darkbite,
            timegem: other.timegem,
            youkai: other.youkai,
            haunted: other.haunted,
            gladiator: other.gladiator,
            sparkblade: other.sparkblade,
            swiftflight: other.swiftflight,
            sacredflame: other.sacredflame,
            ruins: other.ruins,
            lakeshrine: other.lakeshrine,
            glacier: other.glacier,
            memory: other.memory,
            cultist: other.cultist,
            painters: other.painters,
            daynight: other.daynight,
            sharpedge: other.sharpedge,
            oceans: other.oceans,
            performers: other.performers,
            miners: other.miners,
            teaparty: other.teaparty,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum StartingArea {
    RandomKingdom,
    Nest,
    Arsenal,
    Lighthouse,
    Streets,
    Lakeside,
    Depths,
    Sanct,
    Aurum,
    RandomExtra,
    TrueRandom,
    ChaoticRandom,
}

#[wasm_bindgen]
pub fn int_to_area(i: usize) -> StartingArea {
    match i {
        0 => StartingArea::RandomKingdom,
        1 => StartingArea::Nest,
        2 => StartingArea::Arsenal,
        3 => StartingArea::Lighthouse,
        4 => StartingArea::Streets,
        5 => StartingArea::Lakeside,
        6 => StartingArea::Depths,
        7 => StartingArea::Sanct,
        8 => StartingArea::Aurum,
        9 => StartingArea::RandomExtra,
        10 => StartingArea::TrueRandom,
        11 => StartingArea::ChaoticRandom,
        _ => StartingArea::RandomKingdom,
    }
}

impl Into<rnssp::types::StartingArea> for StartingArea {
    fn into(self) -> rnssp::types::StartingArea {
        match self {
            StartingArea::RandomKingdom => rnssp::types::StartingArea::RandomKingdom,
            StartingArea::RandomExtra => rnssp::types::StartingArea::RandomExtra,
            StartingArea::TrueRandom => rnssp::types::StartingArea::TrueRandom,
            StartingArea::ChaoticRandom => rnssp::types::StartingArea::ChaoticRandom,
            StartingArea::Nest => rnssp::types::StartingArea::Nest,
            StartingArea::Arsenal => rnssp::types::StartingArea::Arsenal,
            StartingArea::Lighthouse => rnssp::types::StartingArea::Lighthouse,
            StartingArea::Streets => rnssp::types::StartingArea::Streets,
            StartingArea::Lakeside => rnssp::types::StartingArea::Lakeside,
            StartingArea::Sanct => rnssp::types::StartingArea::Sanct,
            StartingArea::Depths => rnssp::types::StartingArea::Depths,
            StartingArea::Aurum => rnssp::types::StartingArea::Aurum,
        }
    }
}

/// Converts run data to a list of (pointers to) js values, both strings and numbers
fn get_output(run: &Run) -> Vec<JsValue> {
    let areas = run
        .area_list
        .into_iter()
        .map(|area| area as usize)
        .map(|s| JsValue::from_f64(s as f64));
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
    let chest_colors = run
        .chests
        .iter()
        .map(|chest| chest.as_ref().unwrap().color as usize)
        .map(|num| JsValue::from_f64(num as f64));
    once(JsValue::from_f64(run.map_seed as f64))
        .chain(areas)
        .chain(items)
        .chain(shops)
        .chain(chest_colors)
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
    starting_area: StartingArea,
    unlocks: &Unlocks,
) -> Vec<JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    // log(&format!("{:?}", starting_area));
    let mut run = Run::new(
        seed as u32,
        players as u8,
        high_difficulty,
        starting_area.into(),
        unlocks.into(),
    );
    // log(&format!("{:?}", run));
    run.predict_seed();
    // log(&format!("{:?}", run));
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
    starting_area: StartingArea,
    unlocks: &Unlocks,
) -> JsValue {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let data: Vec<Vec<usize>> = unique_seeds
        .into_iter()
        .enumerate()
        .map(|(_i, seed)| {
            // log(&format!("Iteration: {}, seed: {}", i, seed));
            let mut run = Run::new(
                seed as u32,
                players as u8,
                high_difficulty,
                starting_area.into(),
                unlocks.into(),
            );
            run.predict_seed();

            let areas = run.area_list.into_iter().map(|a| a as usize);
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
