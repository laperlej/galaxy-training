mod config;
mod galaxy;
mod manager;

use anyhow::Result;
use crate::manager::TrainingManager;

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 1 {
        println!("Usage: training-manager <config-file>");
        return Ok(());
    }
    let config_path = &args[0];
    let config: config::ConfigFile = config::read_config(config_path)?;

    let galaxy = galaxy::init_galaxy()?;

    let mut training_manager = TrainingManager::new(Box::new(galaxy));

    training_manager.apply_config(&config).await?;

    Ok(())
}

