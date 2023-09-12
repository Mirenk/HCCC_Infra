use chrono::{DateTime, Local};
use postgres_types::ToSql;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Serialize, ToSql)]
pub struct Rank {
    rank: i32,
    userName: String,
    pub score: i64,
    pub time: DateTime<Local>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, ToSql)]
pub struct Ranking {
    status: String,
    #[serde(rename = "items")]
    ranks: Vec<Rank>,
    errorMessage: Option<String>,
}

impl Rank {
    pub fn new(user_name: String, score: i64, time: DateTime<Local>) -> Self {
        Rank {
            rank: 0,
            userName: user_name,
            score,
            time,
        }
    }

    pub fn set_rank(mut self, rank: usize) -> Self {
        self.rank = i32::try_from(rank).expect("cast error: usize -> i32");
        self
    }
}

impl Ranking {
    pub fn new(ranks: Vec<Rank>) -> Self {
        Ranking {
            status: "ok".to_string(),
            ranks,
            errorMessage: None,
        }
    }
}
