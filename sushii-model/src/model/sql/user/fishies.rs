use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::fmt;
use std::ops::Range;

/// Types of fishies, ordered by rareness
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum FishyType {
    Rotten,
    Eel,
    Tilapia,
    Albacore,
    Catfish,
    Sardine,
    Salmon,
    Trout,
    Carp,
    Herring,
    Bream,
    Tuna,
    // Patron fishies
    Dan,
    Jae,
    Joshy,
    Crazy,
    Wawa,
    Tzuwy, // Alonzo's fishy
    Golden,
}

impl FishyType {
    fn fishy_range(&self) -> Range<u64> {
        match self {
            Self::Rotten => 1..10,
            Self::Sardine => 8..14,
            Self::Eel | Self::Tilapia | Self::Albacore | Self::Catfish | Self::Salmon => 8..20,
            Self::Trout => 10..26,
            Self::Carp => 10..25,
            Self::Herring => 8..21,
            Self::Bream => 12..31,
            Self::Tuna => 12..61,
            // Patron fishies
            Self::Dan | Self::Jae | Self::Joshy | Self::Crazy => 15..25,
            Self::Wawa => 20..30,
            Self::Tzuwy => 25..50,
            Self::Golden => 100..180,
        }
    }

    fn common_fishies() -> &'static [FishyType] {
        &[
            Self::Sardine,
            Self::Salmon,
            Self::Carp,
            Self::Herring,
            Self::Bream,
            Self::Tuna,
        ]
    }

    fn patron_fishies() -> &'static [FishyType] {
        &[
            Self::Dan,
            Self::Jae,
            Self::Joshy,
            Self::Crazy,
            Self::Wawa,
            Self::Tzuwy,
        ]
    }

    fn rare_fishies_good() -> &'static [FishyType] {
        &[Self::Golden]
    }

    fn rare_fishies_bad() -> &'static [FishyType] {
        &[Self::Rotten]
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Rotten => "🦴",
            Self::Eel
            | Self::Tilapia
            | Self::Albacore
            | Self::Catfish
            | Self::Sardine
            | Self::Salmon
            | Self::Trout
            | Self::Carp
            | Self::Herring
            | Self::Bream
            | Self::Tuna => "<:fishy:418504956169945089>",
            Self::Dan | Self::Jae | Self::Joshy | Self::Crazy | Self::Tzuwy => {
                "<:fishy:418504956169945089><:fishy:418504956169945089>"
            }
            Self::Wawa => "🍉",
            Self::Golden => "<:goldenFishy:418504966337069057>",
        }
    }

    /// Randomly picks a fishy type, returning the type and amount
    pub fn get_rand_fishies(is_self: bool, is_patron: bool) -> (FishyType, u64) {
        let mut rng = thread_rng();
        // Exclusive of high
        let n: u32 = rng.gen_range(1, 101);

        // Unwrap ok since it's only None when slice is empty
        let fishy_type = if is_patron {
            // Patreon rates
            match n {
                1 => *Self::rare_fishies_bad().choose(&mut rng).unwrap(),
                2..=74 => *Self::common_fishies().choose(&mut rng).unwrap(),
                75..=98 => *Self::patron_fishies().choose(&mut rng).unwrap(),
                // 99 +, 2%
                _ => *Self::rare_fishies_good().choose(&mut rng).unwrap(),
            }
        } else {
            match n {
                1 => *Self::rare_fishies_bad().choose(&mut rng).unwrap(),
                2..=79 => *Self::common_fishies().choose(&mut rng).unwrap(),
                80..=99 => *Self::patron_fishies().choose(&mut rng).unwrap(),
                // 100 +, 1%
                _ => *Self::rare_fishies_good().choose(&mut rng).unwrap(),
            }
        };

        let mut fishy_count: f64 = fishy_type.fishy_range().choose(&mut rng).unwrap() as f64;

        if is_self {
            fishy_count /= 1.8;
        }

        // Only None if iterator is empty
        (fishy_type, fishy_count.round() as u64)
    }
}

impl fmt::Display for FishyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Weird way to lowercase
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn gen_fishies() {
        let mut m = HashMap::new();
        let mut total = 0;

        for _ in 0..100 {
            let (kind, count) = FishyType::get_rand_fishies(false, false);
            total += count;

            // type -> (num catches, average)
            let entry = m.entry(kind).or_insert((0, 0.0));
            //         multiply to get tot  + new   / total count
            entry.1 = ((entry.0 as f64 * entry.1) + count as f64) / (entry.0 + 1) as f64;
            entry.0 += 1;
        }

        println!("total avg: {}, {:#?}", total / 100, m);
    }
}
