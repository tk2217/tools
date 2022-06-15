use crate::util::dfcolor::{COLORS, TemplateColor};
use crate::Tool;
use tera::{Context, Tera};

pub struct DFColorTool;

impl Tool for DFColorTool {
    fn get_name(&self) -> &str {
        "dfcolor"
    }

    fn render(&self, tera: &Tera) -> anyhow::Result<Vec<(String, String)>> {
        let mut context = Context::new();
        context.insert("colors", &COLORS.iter().map(|c| c.clone().into()).collect::<Vec<TemplateColor>>());

        Ok(vec![(
            "index.html".into(),
            tera.render("dfcolor/index.html", &context)?,
        )])
    }
}
