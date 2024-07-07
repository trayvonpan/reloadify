use reloadify::{ConfigId, Format, ReloadableConfig, Reloadify};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::Duration};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TsConfig {
    pub extends: String,
    #[serde(rename = "compilerOptions")]
    pub compiler_options: CompilerOptions,
    pub files: Vec<String>,
    pub include: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompilerOptions {
    #[serde(rename = "outDir")]
    pub out_dir: String,
    pub types: Vec<String>,
}

const TS_CONFIG_ID: &str = "tsconfig";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reloadify = Reloadify::new();

    reloadify.add::<TsConfig>(ReloadableConfig {
        id: ConfigId::new(TS_CONFIG_ID),
        path: Path::new("examples/config/tsconfig.spec.json"),
        format: Format::Json,
        poll_interval: Duration::from_secs(1),
    })?;

    let ts_config = reloadify.get::<TsConfig>(ConfigId::new(TS_CONFIG_ID))?;

    // Do something with ts_config...
    println!("{:?}", ts_config);

    Ok(())
}
