use tera::Tera;
use crate::Tool;

pub struct EmptyTool;

impl Tool for EmptyTool {
    fn get_name(&self) -> &str {
        "empty_tool"
    }

    fn render(&self, _tera: &Tera) -> anyhow::Result<Vec<(String, String)>> {
        Ok(vec![
            ("index.html".to_string(), "Dummy tool to make parcel happy.".to_string())
        ])
    }
}