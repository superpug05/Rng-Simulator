const RARITY_BOUNDARIES: [u32; 9] = [
    1_000_000,
    300_000,
    100_000,
    30_000,
    10_000,
    3_000,
    1_000,
    300,
    50,
];

const RARITY_NAMES: [&str; 9] = [
    "Common",
    "Uncommon",
    "Rare",
    "Epic",
    "Legendary",
    "Mystical",
    "Insane",
    "Zamn!",
    "GO OUTSIDE"
];

const CLASS_BOUNDARIES: [u32; 9] = [
    1_000_000,
    300_000,
    100_000,
    30_000,
    10_000,
    3_000,
    1_000,
    300,
    50,
];

const CLASS_NAMES: [&str; 9] = [
    "Pathetic",
    "Bad",
    "Usable",
    "Decent",
    "Great",
    "Fantastic",
    "Exquisite",
    "Flawless",
    "TOUCH GRASS"
];

pub fn calculate_roll(luck_level: u32, xp: u32) -> u32 {
    let level = (xp as f64/300.0).log(1.03).floor();
    let level = if level == f64::NEG_INFINITY {
        0f64
    } else {
        level
    };
    let roll = fastrand::u32(1..=1_000_000);
    let roll = roll as f64 / (1f64 + luck_level as f64 * 0.05) * (1f64 + level * 0.025);
    roll as u32
}

pub fn get_rarity_name(roll: &u32) -> &str {
    for (index, boundary) in RARITY_BOUNDARIES.iter().enumerate() {
        if roll > boundary {
            return RARITY_NAMES[index-1];
        }
    }
    unreachable!("This cannot happen")
}

pub fn get_class_name(roll: &u32) -> &str {
    for (index, boundary) in CLASS_BOUNDARIES.iter().enumerate() {
        if roll > boundary {
            return CLASS_NAMES[index];
        }
    }
    unreachable!("This cannot happen")
}

pub fn get_rarity_colour(rarity: &str) -> u32 {
    match rarity {
        "Common" => 0xaaaaaa,
        "Uncommon" => 0x20cc20,
        "Rare" => 0x6060ff,
        "Epic" => 0x851485,
        "Legendary" => 0xed9418,
        "Mystical" => 0xff69e6,
        "Insane" => 0x074200,
        "Zamn!" => 0x000000,
        "GO OUTSIDE" => 0xbf0d49,
        _ => unreachable!()
    }
}