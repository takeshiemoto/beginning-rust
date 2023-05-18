use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum SampleError {
    Msg(String),
}

impl Display for SampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::Msg(msg) => write!(f, "SampleError: {}", msg),
        }
    }
}

pub struct Guest {
    age: u32,
    campaign: bool,
}

impl Display for Guest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "年齢:{} キャンペーン:{}", self.age, self.campaign)
    }
}

impl Guest {
    pub fn new(age: u32, campaign: bool) -> Self {
        Self { age, campaign }
    }

    pub fn calc_fee(self) -> Result<u32, SampleError> {
        let fee = match self.age {
            0..=4 => 0,
            5..=12 => 500,
            13..=17 => 700,
            18..=64 => 1000,
            65..=120 => 600,
            _ => return Err(SampleError::Msg(String::from("年齢が不正"))),
        };

        Ok(fee)
    }
}
