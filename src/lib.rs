use tera::Tera;

pub mod tools;

pub trait Tool {
    fn get_name(&self) -> &str;
    fn render(&self, tera: &Tera) -> anyhow::Result<Vec<(String, String)>>;
}
