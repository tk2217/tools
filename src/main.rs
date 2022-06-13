use std::fs;
use std::io;
use std::path::Path;
use anyhow::bail;
use log::info;
use tera::Tera;
use tk2217_tools::tools::dfcolor::DFColorTool;
use tk2217_tools::Tool;
use tk2217_tools::tools::empty_tool::EmptyTool;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let tera = Tera::new("templates/**/*")?;

    let active_tools: Vec<Box<dyn Tool>> = vec![
        Box::from(DFColorTool),
        Box::from(EmptyTool), // Parcel likes to be weird. - Remove once I add more tools.
    ];

    let output_dir = Path::new("./output");
    match fs::remove_dir_all(output_dir) {
        Ok(_) => {}
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {},
                _ => bail!(error)
            }
        }
    }

    fs::create_dir_all(output_dir)?;


    for tool in active_tools {
        info!("Processing tool {}...", tool.get_name());

        let tool_dir = output_dir.join(tool.get_name());
        fs::create_dir_all(&tool_dir)?;

        let results = tool.render(&tera)?;
        for result in results {
            let filename = &result.0;
            let content = &result.1;

            fs::write(tool_dir.join(filename), content)?;
        }
    }

    Ok(())
}
