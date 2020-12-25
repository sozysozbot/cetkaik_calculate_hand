#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::non_ascii_literal)]
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn it_works() {
        use std::convert::TryInto;
        let test_cases = include_str!("test_cases.json");

        let data: Vec<(Vec<String>, AnswerInJSON)> = serde_json::from_str(test_cases).unwrap();

        for (pieces, expected_answer) in data {
            let pieces: Vec<NonTam2Piece> = pieces
                .iter()
                .map(|p| p[..].try_into().unwrap())
                .collect::<Vec<NonTam2Piece>>();

            let answer = calculate_hands_and_score_from_pieces(&pieces);

            assert_eq!(answer, expected_answer.into())
        }
    }
}

use serde::{Deserialize, Serialize};

type ObtainableProf = cetkaik_core::Profession;

use cetkaik_core::{Color, Profession};

impl std::fmt::Display for PositiveHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Io => "王",
                Self::Saup1 => "獣",
                Self::BapPokSaup1 => "同色獣",
                Self::Huep2Hia1 => "地心",
                Self::BapPokHuep2Hia1 => "同色地心",
                Self::Maun1Gua2Kauk2 => "馬弓兵",
                Self::BapPokMaun1Gua2Kauk2 => "同色馬弓兵",
                Self::Uaip2Hi1 => "助友",
                Self::BapPokUaip2Hi1 => "同色助友",
                Self::KaikDat2 => "戦集",
                Self::BapPokKaikDat2 => "同色戦集",
                Self::Mok1Mok1 => "行行",
                Self::BapPokMok1Mok1 => "同色行行",
                Self::Kua2Kauk2Mun1Aum2 => "筆兵無傾",
                Self::BapPokKua2Kauk2Mun1Aum2 => "同色筆兵無傾",
                Self::HuetKaikADat2 => "闇戦之集",
                Self::BapPokHuetKaikADat2 => "同色闇戦之集",
                Self::Mun1Mak1Mok1Hue => "無抗行処",
                Self::BapPokMun1Mak1Mok1Hue => "同色無抗行処",
            }
        )
    }
}

use cetkaik_core::absolute::NonTam2Piece;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PositiveHand {
    /// "王"
    Io,
    /// "獣"
    Saup1,
    /// "同色獣"
    BapPokSaup1,
    /// "地心"
    Huep2Hia1,
    /// "同色地心"
    BapPokHuep2Hia1,
    /// "馬弓兵"
    Maun1Gua2Kauk2,
    /// "同色馬弓兵"
    BapPokMaun1Gua2Kauk2,
    /// "助友"
    Uaip2Hi1,
    /// "同色助友"
    BapPokUaip2Hi1,
    /// "戦集"
    KaikDat2,
    /// "同色戦集"
    BapPokKaikDat2,
    /// "行行"
    Mok1Mok1,
    /// "同色行行"
    BapPokMok1Mok1,
    /// "筆兵無傾"
    Kua2Kauk2Mun1Aum2,
    /// "同色筆兵無傾"
    BapPokKua2Kauk2Mun1Aum2,
    /// "闇戦之集"
    HuetKaikADat2,
    /// "同色闇戦之集"
    BapPokHuetKaikADat2,
    /// "無抗行処"
    Mun1Mak1Mok1Hue,
    /// "同色無抗行処"
    BapPokMun1Mak1Mok1Hue,
}

impl PositiveHand {
    #[allow(clippy::match_same_arms)]
    #[must_use]
    pub const fn hand_to_score(self) -> i32 {
        match self {
            Self::Mun1Mak1Mok1Hue => 50,         // "無抗行処"
            Self::BapPokMun1Mak1Mok1Hue => 52,   // "同色無抗行処"
            Self::Kua2Kauk2Mun1Aum2 => 10,       // "筆兵無傾"
            Self::BapPokKua2Kauk2Mun1Aum2 => 12, // "同色筆兵無傾"
            Self::Huep2Hia1 => 7,                // "地心"
            Self::BapPokHuep2Hia1 => 9,          // "同色地心"
            Self::Maun1Gua2Kauk2 => 5,           // "馬弓兵"
            Self::BapPokMaun1Gua2Kauk2 => 7,     // "同色馬弓兵"
            Self::Mok1Mok1 => 5,                 // "行行"
            Self::BapPokMok1Mok1 => 7,           // "同色行行"
            Self::Io => 3 + 2,                   // "王" /* 同色 by default */
            Self::Saup1 => 3,                    // "獣"
            Self::BapPokSaup1 => 5,              // "同色獣"
            Self::KaikDat2 => 3,                 // "戦集"
            Self::BapPokKaikDat2 => 5,           // "同色戦集"
            Self::Uaip2Hi1 => 3,                 // "助友"
            Self::BapPokUaip2Hi1 => 5,           // "同色助友"
            Self::HuetKaikADat2 => 3,            // "闇戦之集"
            Self::BapPokHuetKaikADat2 => 5,      // "同色闇戦之集"
        }
    }
}

use std::collections::HashSet;

type PieceNumMap = multiset::HashMultiSet<NonTam2Piece>;

fn has(count: &PieceNumMap, prof: ObtainableProf) -> bool {
    count.count_of(&NonTam2Piece {
        prof,
        color: Color::Kok1,
    }) + count.count_of(&NonTam2Piece {
        prof,
        color: Color::Huok2,
    }) > 0
}

fn has_all(count: &PieceNumMap, profs: &[ObtainableProf]) -> bool {
    profs.iter().all(|prof| has(count, *prof))
}

fn has_all_same_color(count: &PieceNumMap, profs: &[ObtainableProf]) -> bool {
    profs.iter().all(|a| {
        count.count_of(&NonTam2Piece {
            prof: *a,
            color: Color::Kok1,
        }) >= 1
    }) || profs.iter().all(|a| {
        count.count_of(&NonTam2Piece {
            prof: *a,
            color: Color::Huok2,
        }) >= 1
    })
}

fn howmany(count: &PieceNumMap, prof: ObtainableProf) -> usize {
    count.count_of(&NonTam2Piece {
        prof,
        color: Color::Kok1,
    }) + count.count_of(&NonTam2Piece {
        prof,
        color: Color::Huok2,
    })
}

fn calculate_hands_with_no_king(count: &PieceNumMap) -> HashSet<PositiveHand> {
    let mut ans: HashSet<PositiveHand> = HashSet::new();
    if count.count_of(&NonTam2Piece {
        prof: Profession::Kauk2,
        color: Color::Kok1,
    }) >= 5
        || count.count_of(&NonTam2Piece {
            prof: Profession::Kauk2,
            color: Color::Huok2,
        }) >= 5
    {
        ans.insert(PositiveHand::BapPokHuetKaikADat2);
    } else if howmany(count, Profession::Kauk2) >= 5 {
        ans.insert(PositiveHand::HuetKaikADat2);
    }
    if (count.count_of(&NonTam2Piece {
        prof: Profession::Kaun1,
        color: Color::Kok1,
    }) >= 1
        && count.count_of(&NonTam2Piece {
            prof: Profession::Kauk2,
            color: Color::Kok1,
        }) >= 2)
        || (count.count_of(&NonTam2Piece {
            prof: Profession::Kaun1,
            color: Color::Huok2,
        }) >= 1
            && count.count_of(&NonTam2Piece {
                prof: Profession::Kauk2,
                color: Color::Huok2,
            }) >= 2)
    {
        ans.insert(PositiveHand::BapPokUaip2Hi1);
    } else if has(count, Profession::Kaun1) && howmany(count, Profession::Kauk2) >= 2 {
        ans.insert(PositiveHand::Uaip2Hi1);
    }
    if (count.count_of(&NonTam2Piece {
        prof: Profession::Uai1,
        color: Color::Kok1,
    }) >= 1
        && count.count_of(&NonTam2Piece {
            prof: Profession::Kauk2,
            color: Color::Kok1,
        }) >= 2)
        || (count.count_of(&NonTam2Piece {
            prof: Profession::Uai1,
            color: Color::Huok2,
        }) >= 1
            && count.count_of(&NonTam2Piece {
                prof: Profession::Kauk2,
                color: Color::Huok2,
            }) >= 2)
    {
        ans.insert(PositiveHand::BapPokKaikDat2);
    } else if has(count, Profession::Uai1) && howmany(count, Profession::Kauk2) >= 2 {
        ans.insert(PositiveHand::KaikDat2);
    }

    f(
        &mut ans,
        count,
        &[Profession::Dau2, Profession::Maun1],
        PositiveHand::BapPokSaup1,
        PositiveHand::Saup1,
    );
    f(
        &mut ans,
        count,
        &[Profession::Nuak1, Profession::Kaun1, Profession::Maun1],
        PositiveHand::BapPokMok1Mok1,
        PositiveHand::Mok1Mok1,
    );
    f(
        &mut ans,
        count,
        &[Profession::Kauk2, Profession::Gua2, Profession::Maun1],
        PositiveHand::BapPokMaun1Gua2Kauk2,
        PositiveHand::Maun1Gua2Kauk2,
    );
    f(
        &mut ans,
        count,
        &[Profession::Kua2, Profession::Tuk2, Profession::Uai1],
        PositiveHand::BapPokHuep2Hia1,
        PositiveHand::Huep2Hia1,
    );
    f(
        &mut ans,
        count,
        &[
            Profession::Kauk2,
            Profession::Gua2,
            Profession::Uai1,
            Profession::Kua2,
            Profession::Tuk2,
        ],
        PositiveHand::BapPokKua2Kauk2Mun1Aum2,
        PositiveHand::Kua2Kauk2Mun1Aum2,
    );

    ans
}

fn f(
    ans: &mut HashSet<PositiveHand>,
    count: &PieceNumMap,
    arr: &[ObtainableProf],
    bappok: PositiveHand,
    hand: PositiveHand,
) {
    if has_all_same_color(count, arr) {
        ans.insert(bappok);
    } else if has_all(count, arr) {
        ans.insert(hand);
    }
}

fn calculate_hands_with_king(count: &PieceNumMap) -> HashSet<PositiveHand> {
    let mut ans: HashSet<PositiveHand> = HashSet::new();
    ans.insert(PositiveHand::Io);

    let prof_list = vec![
        Profession::Kauk2,
        Profession::Gua2,
        Profession::Kaun1,
        Profession::Dau2,
        Profession::Maun1,
        Profession::Kua2,
        Profession::Tuk2,
        Profession::Uai1,
        Profession::Nuak1,
        Profession::Io,
    ];

    if has_all_same_color(count, &prof_list) {
        ans.insert(PositiveHand::BapPokMun1Mak1Mok1Hue);
    } else if has_all(count, &prof_list) {
        ans.insert(PositiveHand::Mun1Mak1Mok1Hue);
    }

    h(&mut ans, count, Color::Kok1);
    h(&mut ans, count, Color::Huok2);

    g(
        &mut ans,
        PositiveHand::BapPokUaip2Hi1,
        PositiveHand::Uaip2Hi1,
    );
    g(
        &mut ans,
        PositiveHand::BapPokHuep2Hia1,
        PositiveHand::Huep2Hia1,
    );
    g(
        &mut ans,
        PositiveHand::BapPokKaikDat2,
        PositiveHand::KaikDat2,
    );
    g(
        &mut ans,
        PositiveHand::BapPokMaun1Gua2Kauk2,
        PositiveHand::Maun1Gua2Kauk2,
    );
    g(&mut ans, PositiveHand::BapPokSaup1, PositiveHand::Saup1);
    g(
        &mut ans,
        PositiveHand::BapPokMok1Mok1,
        PositiveHand::Mok1Mok1,
    );
    g(
        &mut ans,
        PositiveHand::BapPokKua2Kauk2Mun1Aum2,
        PositiveHand::Kua2Kauk2Mun1Aum2,
    );
    g(
        &mut ans,
        PositiveHand::BapPokHuetKaikADat2,
        PositiveHand::HuetKaikADat2,
    );
    g(
        &mut ans,
        PositiveHand::BapPokMun1Mak1Mok1Hue,
        PositiveHand::Mun1Mak1Mok1Hue,
    );

    ans
}

fn g(ans: &mut HashSet<PositiveHand>, flashhand: PositiveHand, hand: PositiveHand) {
    if ans.contains(&flashhand) {
        ans.remove(&hand);
    }
}

fn h(ans: &mut HashSet<PositiveHand>, c: &PieceNumMap, color: Color) {
    let mut count = c.clone();
    let prof_list_excluding_king = vec![
        Profession::Kauk2,
        Profession::Gua2,
        Profession::Kaun1,
        Profession::Dau2,
        Profession::Maun1,
        Profession::Kua2,
        Profession::Tuk2,
        Profession::Uai1,
        Profession::Nuak1,
    ];

    if count.count_of(&NonTam2Piece {
        prof: Profession::Io,
        color,
    }) == 1
    {
        count.remove(&NonTam2Piece {
            prof: Profession::Io,
            color,
        });
        for prof_except_king in prof_list_excluding_king {
            count.insert(NonTam2Piece {
                prof: prof_except_king,
                color,
            }); // wildcard
            for p in calculate_hands_(&count) {
                ans.insert(p);
            }
            count.remove(&NonTam2Piece {
                prof: prof_except_king,
                color,
            });
        }
        count.insert(NonTam2Piece {
            prof: Profession::Io,
            color,
        });
    }
}

fn calculate_hands_(count: &PieceNumMap) -> HashSet<PositiveHand> {
    if count.count_of(&NonTam2Piece {
        prof: Profession::Io,
        color: Color::Huok2,
    }) == 0
        && count.count_of(&NonTam2Piece {
            prof: Profession::Io,
            color: Color::Kok1,
        }) == 0
    {
        calculate_hands_with_no_king(count)
    } else {
        calculate_hands_with_king(count)
    }
}

const fn upper_limit(prof: Profession) -> usize {
    match prof {
        Profession::Kauk2 => 8,
        Profession::Io | Profession::Nuak1 => 1,

        Profession::Gua2
        | Profession::Kaun1
        | Profession::Dau2
        | Profession::Maun1
        | Profession::Kua2
        | Profession::Tuk2
        | Profession::Uai1 => 2,
    }
}

fn calculate_hands_from_pieces(pieces: &[NonTam2Piece]) -> Result<Vec<PositiveHand>, TooMany> {
    let mut count: PieceNumMap = multiset::HashMultiSet::new();
    for p in pieces {
        count.insert(NonTam2Piece {
            prof: p.prof,
            color: p.color,
        });
    }

    // check if the input contains too many pieces
    let mut too_many_list = vec![];

    for color in &[Color::Huok2, Color::Kok1] {
        for prof in &[
            Profession::Kauk2,
            Profession::Gua2,
            Profession::Kaun1,
            Profession::Dau2,
            Profession::Maun1,
            Profession::Kua2,
            Profession::Tuk2,
            Profession::Uai1,
            Profession::Io,
            Profession::Nuak1,
        ] {
            let p = NonTam2Piece {
                prof: *prof,
                color: *color,
            };
            if count.count_of(&p) > upper_limit(*prof) {
                too_many_list.push(p.to_string());
            }
        }
    }

    if !too_many_list.is_empty() {
        return Err(TooMany(too_many_list));
    }
    return Ok(calculate_hands_(&count).iter().copied().collect());
}

/// # Errors
/// Fails when the input is impossible (e.g. there are three or more Io)
pub fn calculate_hands_and_score_from_pieces(ps: &[NonTam2Piece]) -> Answer {
    match calculate_hands_from_pieces(ps) {
        Err(TooMany(too_many_list)) => Err(TooMany(too_many_list)),
        Ok(hands) => Ok(ScoreAndHands {
            score: hands.iter().map(|h| h.hand_to_score()).sum(),
            hands: hands.iter().map(std::string::ToString::to_string).collect(),
        }),
    }
}

type Answer = Result<ScoreAndHands, TooMany>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum AnswerInJSON {
    ErrorTrue {
        error: bool, /* true */
        too_many: Vec<String>,
    },
    ErrorFalse {
        error: bool, /* false */
        score: i32,
        hands: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreAndHands {
    pub score: i32,
    pub hands: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TooMany(Vec<String>);

impl Into<Answer> for AnswerInJSON {
    fn into(self) -> Result<ScoreAndHands, TooMany> {
        match self {
            Self::ErrorTrue { error, too_many } => {
                if !error {
                    panic!("Invalid AnswerInJSON: has field `too_many` but the value of `error` is false");
                }
                Err(TooMany(too_many))
            }
            Self::ErrorFalse {
                error,
                score,
                hands,
            } => {
                if error {
                    panic!("Invalid AnswerInJSON: has field `score` and `hands` but the value of `error` is true");
                }
                Ok(ScoreAndHands {
                    score,
                    hands: hands.into_iter().collect(),
                })
            }
        }
    }
}
