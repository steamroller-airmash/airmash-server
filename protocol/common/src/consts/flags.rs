use crate::enums::FlagCode;

pub fn str_to_flag(s: &str) -> Option<FlagCode> {
    str_to_flag_inner(s)
}

fn str_to_flag_inner(s: &str) -> Option<FlagCode> {
    use self::FlagCode::*;

    match s.len() {
        5 => return Some(JollyRogers).filter(|_| uppercmp(s, "JOLLY")),
        7 => return Some(Rainbow).filter(|_| uppercmp(s, "RAINBOW")),
        8 => return Some(ImperialJapan).filter(|_| uppercmp(s, "IMPERIAL")),
        9 => return Some(Communist).filter(|_| uppercmp(s, "COMMUNIST")),
        11 => return Some(Confederate).filter(|_| uppercmp(s, "CONFEDERATE")),
        2 => (),
        _ => return None,
    }

    // We only care about letters. This maps all lowercase
    // letters to uppercase without mapping any other letter
    // to an uppercase one.
    let c1 = s.as_bytes()[0] & !0x20u8;
    let c2 = s.as_bytes()[1] & !0x20u8;

    let flag = match c1 {
        b'A' => match c2 {
            b'D' => Andorra,
            b'E' => UnitedArabEmirates,
            b'L' => Albania,
            b'M' => Armenia,
            b'Q' => Antarctica,
            b'R' => Australia,
            b'T' => Austria,
            b'U' => Australia,
            b'Z' => Azerbaijan,
            _ => return None,
        },
        b'B' => match c2 {
            b'A' => BosniaAndHerzegovina,
            b'D' => Bangladesh,
            b'E' => Belgium,
            b'G' => Bulgaria,
            b'H' => Bahrain,
            b'O' => Bolivia,
            b'R' => Brazil,
            b'T' => Bhutan,
            b'Y' => Belarus,
            _ => return None,
        },
        b'C' => match c2 {
            b'A' => Canada,
            b'H' => Switzerland,
            b'L' => Chile,
            b'N' => China,
            b'O' => Colombia,
            b'R' => CostaRica,
            b'U' => Cuba,
            b'Y' => Cyprus,
            b'Z' => CzechRepublic,
            _ => return None,
        },
        b'D' => match c2 {
            b'E' => Germany,
            b'K' => Denmark,
            b'O' => DominicanRepublic,
            b'Z' => Algeria,
            _ => return None,
        },
        b'E' => match c2 {
            b'C' => Ecuador,
            b'E' => Estonia,
            b'G' => Egypt,
            b'S' => Spain,
            b'U' => EuropeanUnion,
            _ => return None,
        },
        b'F' => match c2 {
            b'I' => Finland,
            b'R' => France,
            _ => return None,
        },
        b'G' => match c2 {
            b'B' => UnitedKingdom,
            b'E' => Georgia,
            b'R' => Greece,
            b'T' => Guatemala,
            _ => return None,
        },
        b'H' => match c2 {
            b'K' => HongKong,
            b'N' => Honduras,
            b'R' => Croatia,
            b'U' => Hungary,
            _ => return None,
        },
        b'I' => match c2 {
            b'D' => Indonesia,
            b'E' => Ireland,
            b'L' => Israel,
            b'M' => IsleOfMan,
            b'N' => India,
            b'Q' => Iraq,
            b'R' => Iran,
            b'S' => Iceland,
            b'T' => Italy,
            _ => return None,
        },
        b'J' => match c2 {
            b'M' => Jamaica,
            b'O' => Jordan,
            b'P' => Japan,
            _ => return None,
        },
        b'K' => match c2 {
            b'P' => DPRK,
            b'R' => SouthKorea,
            b'W' => Kuwait,
            b'Z' => Kazakhstan,
            _ => return None,
        },
        b'L' => match c2 {
            b'B' => Lebanon,
            b'I' => Liechtenstein,
            b'K' => SriLanka,
            b'T' => Lithuania,
            b'U' => Luxembourg,
            b'V' => Latvia,
            b'Y' => LibyanArabJamahiriya,
            _ => return None,
        },
        b'M' => match c2 {
            b'A' => Morocco,
            b'C' => Monaco,
            b'D' => Moldova,
            b'E' => Montenegro,
            b'K' => Macedonia,
            b'O' => Macao,
            b'T' => Malta,
            b'X' => Mexico,
            b'Y' => Malaysia,
            _ => return None,
        },
        b'N' => match c2 {
            b'G' => Nigeria,
            b'L' => Netherlands,
            b'O' => Norway,
            b'P' => Nepal,
            b'Z' => NewZealand,
            _ => return None,
        },
        b'O' => match c2 {
            b'M' => Oman,
            _ => return None,
        },
        b'P' => match c2 {
            b'A' => Panama,
            b'E' => Peru,
            b'K' => Pakistan,
            b'L' => Poland,
            b'H' => Philippines,
            b'R' => PuertoRico,
            b'T' => Portugal,
            b'Y' => Paraguay,
            _ => return None,
        },
        b'Q' => match c2 {
            b'A' => Qatar,
            _ => return None,
        },
        b'R' => match c2 {
            b'O' => Romania,
            b'S' => Serbia,
            b'U' => RussianFederation,
            _ => return None,
        },
        b'S' => match c2 {
            b'A' => SaudiArabia,
            b'E' => Sweden,
            b'G' => Singapore,
            b'I' => Slovenia,
            b'K' => Slovakia,
            b'M' => SanMarino,
            b'O' => Somalia,
            b'V' => ElSalvador,
            b'Y' => SyrianArabRepublic,
            _ => return None,
        },
        b'T' => match c2 {
            b'H' => Thailand,
            b'N' => Tunisia,
            b'M' => Turkmenistan,
            b'R' => Turkey,
            b'T' => TrinidadAndTobago,
            b'W' => Taiwan,
            b'Z' => Tanzania,
            _ => return None,
        },
        b'U' => match c2 {
            b'A' => Ukraine,
            b'N' => UnitedNations,
            b'S' => UnitedStates,
            b'Y' => Uruguay,
            b'Z' => Uzbekistan,
            _ => return None,
        },
        b'V' => match c2 {
            b'E' => Venezuela,
            b'N' => Vietnam,
            _ => return None,
        },
        b'Z' => match c2 {
            b'A' => SouthAfrica,
            _ => return None,
        },
        _ => return None,
    };

    Some(flag)
}

fn uppercmp(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.as_bytes()
        .iter()
        .copied()
        .zip(b.as_bytes().iter().copied())
        .all(|(a, b): (u8, u8)| a.to_ascii_uppercase() == b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_str_to_flag_lookup(b: &mut Bencher) {
        b.iter(|| {
            let s = black_box("CA");
            str_to_flag(s)
        })
    }
}
