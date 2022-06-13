use log::{debug, info};
use std::fs;
use std::path::Path;
use tera::Tera;
use tk2217_tools::tools::dfcolor::DFColorTool;
use tk2217_tools::{copy_all_from_dir, remove_dir_if_exists, Tool};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let tera = Tera::new("templates/**/*")?;

    let active_tools: Vec<Box<dyn Tool>> = vec![
        Box::from(DFColorTool),
    ];

    let output_dir = Path::new("./output");
    remove_dir_if_exists(output_dir)?;

    fs::create_dir_all(output_dir)?;

    info!("Building tools...");
    for tool in active_tools {
        info!("Building tool {}...", tool.get_name());

        let tool_dir = output_dir.join(tool.get_name());
        fs::create_dir_all(&tool_dir)?;

        let results = tool.render(&tera)?;
        for result in results {
            let filename = &result.0;
            let content = &result.1;

            debug!("Writing file {}.", filename);
            fs::write(tool_dir.join(filename), content)?;
        }

        info!("Finished building tool {}.", tool.get_name());
    }
    info!("Finished building tools.");

    info!("Assembling tools...");
    let static_dir = Path::new("./static");
    let dist_dir = Path::new("./dist");
    remove_dir_if_exists(dist_dir)?;

    copy_all_from_dir(output_dir, dist_dir)?;
    copy_all_from_dir(static_dir, dist_dir)?;

    info!("Finished assembling tools.");

    Ok(())
}
