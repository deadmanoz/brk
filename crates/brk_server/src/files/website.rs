use clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, ValueEnum)]
pub enum Website {
    None,
    Default,
    Custom,
}

impl Website {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn to_folder_name(&self) -> &str {
        match self {
            Self::Custom => "custom",
            Self::Default => "default",
            Self::None => unreachable!(),
        }
    }
}
