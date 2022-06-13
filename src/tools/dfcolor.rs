use crate::Tool;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use tera::{Context, Tera};

const NAMED_COLORS: Lazy<HashMap<String, DFColor>> = Lazy::new(|| {
    include_str!("../../data/named_colors.csv")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.splitn(3, ",").collect::<Vec<&str>>())
        .map(|e| {
            if let [name, hex, legacy] = e[..] {
                DFColor {
                    name: name.to_string(),
                    value: u32::from_str_radix(hex, 16).expect("Hex color"),
                    hex: hex.to_string(),
                    legacy: format!("&{}", legacy),
                }
            } else {
                panic!("Invalid named color {:?}!", e);
            }
        })
        .map(|e| (e.name.clone(), e))
        .collect()
});

const COLORS: Lazy<Vec<DFColor>> = Lazy::new(|| {
    include_str!("../../data/dfcolor.txt")
        .split("\n")
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once("="))
        .filter_map(|(name, color)| {
            Some(
                if let Some(named_color) = color.strip_prefix("NamedTextColor.") {
                    // I suspect that this is a borrow checker bug, although this fix shouldn't have any performance implications.
                    // TODO: Ask around about this.
                    let named_colors = NAMED_COLORS;
                    let named_color = named_colors.get(named_color).expect("Named color");

                    DFColor {
                        name: name.to_string(),
                        value: named_color.value,
                        hex: named_color.hex.clone(),
                        legacy: named_color.legacy.clone(),
                    }
                } else {
                    let octets: Vec<u8> = color
                        .splitn(3, ",")
                        .map(|s| u8::from_str_radix(s, 10).expect("color channel"))
                        .collect();

                    if let [r, g, b] = octets[..] {
                        let value = u32::from_be_bytes([0, r, g, b]);
                        let hex = format!("{:06x}", value);
                        let legacy = format!("&x&{}", hex.chars().map(|e| e.to_string()).collect::<Vec<String>>().join("&"));

                        DFColor {
                            name: name.to_string(),
                            value,
                            hex,
                            legacy,
                        }
                    } else {
                        panic!("Invalid color spec {:?}!", octets);
                    }
                },
            )
        })
        .collect()
});

#[derive(Eq, PartialEq, Deserialize, Serialize)]
struct DFColor {
    name: String,
    value: u32,
    hex: String,
    legacy: String,
}

pub struct DFColorTool;

impl Tool for DFColorTool {
    fn get_name(&self) -> &str {
        "dfcolor"
    }

    fn render(&self, tera: &Tera) -> anyhow::Result<Vec<(String, String)>> {
        let mut context = Context::new();
        context.insert("colors", COLORS.deref());

        Ok(vec![(
            "index.html".into(),
            tera.render("dfcolor/index.html", &context)?,
        )])
    }
}
