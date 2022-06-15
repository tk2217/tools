use std::str::FromStr;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::util::NamedColor;

pub const COLORS: Lazy<Vec<DFColor>> = Lazy::new(|| {
    include_str!("../../data/dfcolor.csv")
        .split("\n")
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once(","))
        .filter_map(|(name, color)| {
            let value = if let Some(named_color) = color.strip_prefix("NamedTextColor.") {
                NamedColor::from_str(named_color).ok()?.into()
            } else {
                u32::from_str_radix(color, 16).ok()?
            };

            Some(DFColor {
                name: name.to_string(),
                value,
            })
        })
        .collect()
});

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct DFColor {
    name: String,
    value: u32,
}

#[derive(Eq, PartialEq, Deserialize, Serialize)]
pub struct TemplateColor {
    pub color: DFColor,
    pub hex: String,
    pub legacy: String,
}

impl From<DFColor> for TemplateColor {
    fn from(color: DFColor) -> Self {
        let hex = format!("{:06x}", color.value);
        let legacy = format!("&x{}", hex.chars().map(|c| format!("&{}", c)).collect::<String>());

        TemplateColor {
            color,
            hex,
            legacy,
        }
    }
}