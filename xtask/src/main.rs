#![allow(clippy::suspicious_else_formatting)]

use std::{collections::BTreeMap, fs, process};

use anyhow::Context;
use clap::Parser;
use lumeo_api_client::pipeline::Pipeline;
use serde_json::Value as JsonValue;

#[derive(Parser)]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// Checks whether a JSON pipeline definition can be deserialized correctly
    CheckPipeline { pipeline_file: String, configuration_file: Option<String> },
    /// Merges configuration options into a pipeline definition
    ConfigurePipeline { pipeline_file: String, configuration_file: String },
}

fn main() {
    let opt = Options::parse();
    let res = match opt.subcmd {
        SubCommand::CheckPipeline { pipeline_file, configuration_file } => {
            check_pipeline(&pipeline_file, configuration_file.as_deref())
        }
        SubCommand::ConfigurePipeline { pipeline_file, configuration_file } => {
            configure_pipeline(&pipeline_file, &configuration_file)
        }
    };

    if let Err(e) = res {
        eprintln!("{}", e);
        process::exit(1);
    }
}

type Configuration = BTreeMap<String, serde_json::Map<String, JsonValue>>;

fn check_pipeline(pipeline_file: &str, configuration_file: Option<&str>) -> anyhow::Result<()> {
    let mut pipeline_json = fs::read_to_string(pipeline_file)?;

    if let Some(file) = configuration_file {
        let configuration_json = fs::read_to_string(file)?;
        let configuration = match serde_json::from_str::<Configuration>(&configuration_json) {
            Ok(config) => config,
            Err(e) => {
                eprint!("Invalid pipeline configuration: ");
                return Err(e.into());
            }
        };

        update_pipeline_def(&mut pipeline_json, &configuration).context("configuring pipeline")?;
    }

    match serde_json::from_str::<Pipeline>(&pipeline_json) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("Invalid pipeline definition: ");
            Err(e.into())
        }
    }
}

fn configure_pipeline(pipeline_file: &str, configuration_file: &str) -> Result<(), anyhow::Error> {
    let mut pipeline_json = fs::read_to_string(pipeline_file)?;
    let configuration_json = fs::read_to_string(configuration_file)?;

    let configuration = match serde_json::from_str::<Configuration>(&configuration_json) {
        Ok(config) => config,
        Err(e) => {
            eprint!("Invalid pipeline configuration: ");
            return Err(e.into());
        }
    };

    update_pipeline_def(&mut pipeline_json, &configuration)?;

    println!("{}", pipeline_json);
    Ok(())
}

const DYNAMIC_NODES: &[&str] = &[
    "annotate_barcode",
    "annotate_line_counter",
    "annotate_lpr",
    "annotate_motion",
    "annotate_presence",
    "annotate_queue",
    "filter_frames",
    "log_meta",
    "overlay_meta",
    "publish_google_sheets",
    "stream_rtmp",
    "transform_blur",
    "watermark",
    "webhook_local",
];

fn update_pipeline_def(
    pipeline_json: &mut String,
    configuration: &BTreeMap<String, serde_json::Map<String, JsonValue>>,
) -> anyhow::Result<()> {
    let mut pipeline_nodes: Vec<JsonValue> = serde_json::from_str(pipeline_json)?;
    pipeline_nodes.retain(|node| {
        let props_obj = node["properties"].as_object().expect("properties is not an object");
        let node_type = props_obj["type"].as_str().expect("Couldn't obtain node type");
        !DYNAMIC_NODES.contains(&node_type)
    });

    for node in &mut pipeline_nodes {
        let node_id = node["id"].as_str().context("id is not a string")?.to_owned();
        let props_obj =
            node["properties"].as_object_mut().context("properties is not an object")?;

        if let Some(config_props) = configuration.get(&node_id) {
            props_obj.extend(config_props.clone());
        }
    }

    *pipeline_json = serde_json::to_string(&pipeline_nodes)?;

    Ok(())
}
