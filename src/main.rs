use log::error;
use std::fs;
use std::path::Path;
use tera::Tera;
use tk2217_tools::tools::dfcolor::DFColorTool;
use tk2217_tools::Tool;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            error!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let active_tools = vec![
        DFColorTool,
    ];

    let output_dir = Path::new("./output");
    fs::remove_dir_all(output_dir)?;
    fs::create_dir_all(output_dir)?;

    for tool in active_tools {
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
