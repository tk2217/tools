use tera::Tera;

pub mod color;
pub mod tools;
pub mod util;

pub trait Tool {
    fn get_name(&self) -> &str;
    fn render(&self, tera: &Tera) -> anyhow::Result<Vec<(String, String)>>;
}
