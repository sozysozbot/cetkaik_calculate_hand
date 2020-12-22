#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn it_works() {
        let test_cases = include_str!("test_cases.json");

        let data: Vec<(Vec<String>, AnswerInJSON)> = serde_json::from_str(test_cases).unwrap();

        for (pieces, expected_answer) in data {
            let pieces: Vec<ObtainablePieces2> = pieces
            .iter()
            .map(|p| p[..].try_into().unwrap())
            .collect::<Vec<ObtainablePieces2>>();

            let answer = calculate_hands_and_score_from_pieces(&pieces);

            assert_eq!(answer, expected_answer.into())
        }
    }
}

use serde::{Deserialize, Serialize};

type ObtainableProf = cetkaik_core::Profession;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct ObtainablePieces2 {
    color: cetkaik_core::Color,
    prof: ObtainableProf,
}

use cetkaik_core::{Color, Profession};
use std::convert::TryInto;

impl TryInto<ObtainablePieces2> for &str {
    type Error = ();
    fn try_into(self) -> Result<ObtainablePieces2, Self::Error> {
        Ok(match self {
            "黒兵" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Kauk2,
            },
            "赤兵" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Kauk2,
            },
            "黒弓" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Gua2,
            },
            "黒車" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Kaun1,
            },
            "黒虎" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Dau2,
            },
            "黒馬" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Maun1,
            },
            "黒筆" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Kua2,
            },
            "黒巫" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Tuk2,
            },
            "黒将" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Uai1,
            },
            "赤弓" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Gua2,
            },
            "赤車" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Kaun1,
            },
            "赤虎" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Dau2,
            },
            "赤馬" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Maun1,
            },
            "赤筆" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Kua2,
            },
            "赤巫" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Tuk2,
            },
            "赤将" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Uai1,
            },
            "黒王" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Io,
            },
            "赤王" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Io,
            },
            "黒船" => ObtainablePieces2 {
                color: Color::Huok2,
                prof: Profession::Nuak1,
            },
            "赤船" => ObtainablePieces2 {
                color: Color::Kok1,
                prof: Profession::Nuak1,
            },
            _ => return Err(()),
        })
    }
}

impl std::fmt::Display for PositiveHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                PositiveHand::Io => "王",
                PositiveHand::Saup1 => "獣",
                PositiveHand::BapPokSaup1 => "同色獣",
                PositiveHand::Huep2Hia1 => "地心",
                PositiveHand::BapPokHuep2Hia1 => "同色地心",
                PositiveHand::Maun1Gua2Kauk2 => "馬弓兵",
                PositiveHand::BapPokMaun1Gua2Kauk2 => "同色馬弓兵",
                PositiveHand::Uaip2Hi1 => "助友",
                PositiveHand::BapPokUaip2Hi1 => "同色助友",
                PositiveHand::KaikDat2 => "戦集",
                PositiveHand::BapPokKaikDat2 => "同色戦集",
                PositiveHand::Mok1Mok1 => "行行",
                PositiveHand::BapPokMok1Mok1 => "同色行行",
                PositiveHand::Kua2Kauk2Mun1Aum2 => "筆兵無傾",
                PositiveHand::BapPokKua2Kauk2Mun1Aum2 => "同色筆兵無傾",
                PositiveHand::HuetKaikADat2 => "闇戦之集",
                PositiveHand::BapPokHuetKaikADat2 => "同色闇戦之集",
                PositiveHand::Mun1Mak1Mok1Hue => "無抗行処",
                PositiveHand::BapPokMun1Mak1Mok1Hue => "同色無抗行処",
            }
        )
    }
}

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
    pub fn hand_to_score(self) -> i32 {
        match self {
            PositiveHand::Mun1Mak1Mok1Hue => 50,         // "無抗行処"
            PositiveHand::BapPokMun1Mak1Mok1Hue => 52,   // "同色無抗行処"
            PositiveHand::Kua2Kauk2Mun1Aum2 => 10,       // "筆兵無傾"
            PositiveHand::BapPokKua2Kauk2Mun1Aum2 => 12, // "同色筆兵無傾"
            PositiveHand::Huep2Hia1 => 7,                // "地心"
            PositiveHand::BapPokHuep2Hia1 => 9,          // "同色地心"
            PositiveHand::Maun1Gua2Kauk2 => 5,           // "馬弓兵"
            PositiveHand::BapPokMaun1Gua2Kauk2 => 7,     // "同色馬弓兵"
            PositiveHand::Mok1Mok1 => 5,                 // "行行"
            PositiveHand::BapPokMok1Mok1 => 7,           // "同色行行"
            PositiveHand::Io => 3 + 2,                   // "王" /* 同色 by default */
            PositiveHand::Saup1 => 3,                    // "獣"
            PositiveHand::BapPokSaup1 => 5,              // "同色獣"
            PositiveHand::KaikDat2 => 3,                 // "戦集"
            PositiveHand::BapPokKaikDat2 => 5,           // "同色戦集"
            PositiveHand::Uaip2Hi1 => 3,                 // "助友"
            PositiveHand::BapPokUaip2Hi1 => 5,           // "同色助友"
            PositiveHand::HuetKaikADat2 => 3,            // "闇戦之集"
            PositiveHand::BapPokHuetKaikADat2 => 5,      // "同色闇戦之集"
        }
    }
}

use std::collections::HashSet;

type PieceNumMap = multiset::HashMultiSet<ObtainablePieces2>;

fn has(count: &PieceNumMap, prof: ObtainableProf) -> bool {
    return count.count_of(&ObtainablePieces2 {
        prof: prof,
        color: Color::Kok1,
    }) + count.count_of(&ObtainablePieces2 {
        prof: prof,
        color: Color::Huok2,
    }) > 0;
}

fn has_all(count: &PieceNumMap, profs: &[ObtainableProf]) -> bool {
    return profs.iter().all(|prof| has(&count, *prof));
}

fn has_all_same_color(count: &PieceNumMap, profs: &[ObtainableProf]) -> bool {
    return profs.iter().all(|a| {
        count.count_of(&ObtainablePieces2 {
            prof: *a,
            color: Color::Kok1,
        }) >= 1
    }) || profs.iter().all(|a| {
        count.count_of(&ObtainablePieces2 {
            prof: *a,
            color: Color::Huok2,
        }) >= 1
    });
}

fn howmany(count: &PieceNumMap, prof: ObtainableProf) -> usize {
    return count.count_of(&ObtainablePieces2 {
        prof: prof,
        color: Color::Kok1,
    }) + count.count_of(&ObtainablePieces2 {
        prof: prof,
        color: Color::Huok2,
    });
}

fn calculate_hands_with_no_king(count: &PieceNumMap) -> HashSet<PositiveHand> {
    let mut ans: HashSet<PositiveHand> = HashSet::new();
    if count.count_of(&ObtainablePieces2 {
        prof: Profession::Kauk2,
        color: Color::Kok1,
    }) >= 5
        || count.count_of(&ObtainablePieces2 {
            prof: Profession::Kauk2,
            color: Color::Huok2,
        }) >= 5
    {
        ans.insert(PositiveHand::BapPokHuetKaikADat2);
    } else if howmany(&count, Profession::Kauk2) >= 5 {
        ans.insert(PositiveHand::HuetKaikADat2);
    }
    if (count.count_of(&ObtainablePieces2 {
        prof: Profession::Kaun1,
        color: Color::Kok1,
    }) >= 1
        && count.count_of(&ObtainablePieces2 {
            prof: Profession::Kauk2,
            color: Color::Kok1,
        }) >= 2)
        || (count.count_of(&ObtainablePieces2 {
            prof: Profession::Kaun1,
            color: Color::Huok2,
        }) >= 1
            && count.count_of(&ObtainablePieces2 {
                prof: Profession::Kauk2,
                color: Color::Huok2,
            }) >= 2)
    {
        ans.insert(PositiveHand::BapPokUaip2Hi1);
    } else if has(&count, Profession::Kaun1) && howmany(&count, Profession::Kauk2) >= 2 {
        ans.insert(PositiveHand::Uaip2Hi1);
    }
    if (count.count_of(&ObtainablePieces2 {
        prof: Profession::Uai1,
        color: Color::Kok1,
    }) >= 1
        && count.count_of(&ObtainablePieces2 {
            prof: Profession::Kauk2,
            color: Color::Kok1,
        }) >= 2)
        || (count.count_of(&ObtainablePieces2 {
            prof: Profession::Uai1,
            color: Color::Huok2,
        }) >= 1
            && count.count_of(&ObtainablePieces2 {
                prof: Profession::Kauk2,
                color: Color::Huok2,
            }) >= 2)
    {
        ans.insert(PositiveHand::BapPokKaikDat2);
    } else if has(&count, Profession::Uai1) && howmany(&count, Profession::Kauk2) >= 2 {
        ans.insert(PositiveHand::KaikDat2);
    }

    f(
        &mut ans,
        &count,
        &[Profession::Dau2, Profession::Maun1],
        PositiveHand::BapPokSaup1,
        PositiveHand::Saup1,
    );
    f(
        &mut ans,
        &count,
        &[Profession::Nuak1, Profession::Kaun1, Profession::Maun1],
        PositiveHand::BapPokMok1Mok1,
        PositiveHand::Mok1Mok1,
    );
    f(
        &mut ans,
        &count,
        &[Profession::Kauk2, Profession::Gua2, Profession::Maun1],
        PositiveHand::BapPokMaun1Gua2Kauk2,
        PositiveHand::Maun1Gua2Kauk2,
    );
    f(
        &mut ans,
        &count,
        &[Profession::Kua2, Profession::Tuk2, Profession::Uai1],
        PositiveHand::BapPokHuep2Hia1,
        PositiveHand::Huep2Hia1,
    );
    f(
        &mut ans,
        &count,
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

    return ans;
}

fn f(
    ans: &mut HashSet<PositiveHand>,
    count: &PieceNumMap,
    arr: &[ObtainableProf],
    bappok: PositiveHand,
    hand: PositiveHand,
) {
    if has_all_same_color(&count, arr) {
        ans.insert(bappok);
    } else if has_all(&count, arr) {
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

    h(&mut ans, &count, Color::Kok1);
    h(&mut ans, &count, Color::Huok2);

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

    return ans;
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

    if count.count_of(&ObtainablePieces2 {
        prof: Profession::Io,
        color: color,
    }) == 1
    {
        count.remove(&ObtainablePieces2 {
            prof: Profession::Io,
            color: color,
        });
        for prof_except_king in prof_list_excluding_king {
            count.insert(ObtainablePieces2 {
                prof: prof_except_king,
                color,
            }); // wildcard
            for p in calculate_hands_(&count) {
                ans.insert(p);
            }
            count.remove(&ObtainablePieces2 {
                prof: prof_except_king,
                color,
            });
        }
        count.insert(ObtainablePieces2 {
            prof: Profession::Io,
            color: color,
        });
    }
}

fn calculate_hands_(count: &PieceNumMap) -> HashSet<PositiveHand> {
    if count.count_of(&ObtainablePieces2 {
        prof: Profession::Io,
        color: Color::Huok2,
    }) == 0
        && count.count_of(&ObtainablePieces2 {
            prof: Profession::Io,
            color: Color::Kok1,
        }) == 0
    {
        return calculate_hands_with_no_king(&count);
    } else {
        return calculate_hands_with_king(&count);
    }
}

fn upper_limit(prof: Profession) -> usize {
    match prof {
        Profession::Kauk2 => 8,
        Profession::Gua2 => 2,
        Profession::Kaun1 => 2,
        Profession::Dau2 => 2,
        Profession::Maun1 => 2,
        Profession::Kua2 => 2,
        Profession::Tuk2 => 2,
        Profession::Uai1 => 2,
        Profession::Io => 1,
        Profession::Nuak1 => 1,
    }
}

fn calculate_hands_from_pieces(pieces: &[ObtainablePieces2]) -> Result<Vec<PositiveHand>, TooMany> {
    let mut count: PieceNumMap = multiset::HashMultiSet::new();
    for p in pieces {
        count.insert(ObtainablePieces2 {
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
            let p = ObtainablePieces2 {
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
    return Ok(calculate_hands_(&count).iter().map(|p| *p).collect());
}

impl std::fmt::Display for ObtainablePieces2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match *self {
            ObtainablePieces2 {
                prof: Profession::Kauk2,
                color: Color::Huok2,
            } => "黒兵",
            ObtainablePieces2 {
                prof: Profession::Kauk2,
                color: Color::Kok1,
            } => "赤兵",
            ObtainablePieces2 {
                prof: Profession::Gua2,
                color: Color::Huok2,
            } => "黒弓",
            ObtainablePieces2 {
                prof: Profession::Gua2,
                color: Color::Kok1,
            } => "赤弓",
            ObtainablePieces2 {
                prof: Profession::Kaun1,
                color: Color::Huok2,
            } => "黒車",
            ObtainablePieces2 {
                prof: Profession::Kaun1,
                color: Color::Kok1,
            } => "赤車",
            ObtainablePieces2 {
                prof: Profession::Dau2,
                color: Color::Huok2,
            } => "黒虎",
            ObtainablePieces2 {
                prof: Profession::Dau2,
                color: Color::Kok1,
            } => "赤虎",
            ObtainablePieces2 {
                prof: Profession::Maun1,
                color: Color::Huok2,
            } => "黒馬",
            ObtainablePieces2 {
                prof: Profession::Maun1,
                color: Color::Kok1,
            } => "赤馬",
            ObtainablePieces2 {
                prof: Profession::Kua2,
                color: Color::Huok2,
            } => "黒筆",
            ObtainablePieces2 {
                prof: Profession::Kua2,
                color: Color::Kok1,
            } => "赤筆",
            ObtainablePieces2 {
                prof: Profession::Tuk2,
                color: Color::Huok2,
            } => "黒巫",
            ObtainablePieces2 {
                prof: Profession::Tuk2,
                color: Color::Kok1,
            } => "赤巫",
            ObtainablePieces2 {
                prof: Profession::Uai1,
                color: Color::Huok2,
            } => "黒将",
            ObtainablePieces2 {
                prof: Profession::Uai1,
                color: Color::Kok1,
            } => "赤将",
            ObtainablePieces2 {
                prof: Profession::Io,
                color: Color::Huok2,
            } => "黒王",
            ObtainablePieces2 {
                prof: Profession::Io,
                color: Color::Kok1,
            } => "赤王",
            ObtainablePieces2 {
                prof: Profession::Nuak1,
                color: Color::Huok2,
            } => "黒船",
            ObtainablePieces2 {
                prof: Profession::Nuak1,
                color: Color::Kok1,
            } => "赤船",
        };
        write!(f, "{}", a)
    }
}

pub fn calculate_hands_and_score_from_pieces(ps: &[ObtainablePieces2]) -> Answer {
    match calculate_hands_from_pieces(ps) {
        Err(TooMany(too_many_list)) => Err(TooMany(too_many_list)),
        Ok(hands) => Ok(ScoreAndHands {
            score: hands.iter().map(|h| h.hand_to_score()).sum(),
            hands: hands.iter().map(|h| h.to_string()).collect(),
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
            AnswerInJSON::ErrorTrue { error, too_many } => {
                if !error {
                    panic!("Invalid AnswerInJSON: has field `too_many` but the value of `error` is false");
                }
                Err(TooMany(too_many))
            }
            AnswerInJSON::ErrorFalse {
                error,
                score,
                hands,
            } => {
                if error {
                    panic!("Invalid AnswerInJSON: has field `score` and `hands` but the value of `error` is true");
                }
                Ok(ScoreAndHands { score, hands: hands.into_iter().collect() })
            }
        }
    }
}
