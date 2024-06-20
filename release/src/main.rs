
use base64::prelude::*;
use colored::Colorize;
use log::{error, info, warn, LevelFilter};
use std::fs::File;
use std::io::Write;

mod options;
mod utils;
mod x86_64;
mod arm;
mod plugin;
mod visualization;
mod reporting;
mod example_plugins;

use plugin::{PluginManager, TransformationPlugin};
use visualization::VisualizationTool;
use reporting::ReportingModule;

struct ExampleTransformationPlugin;

impl TransformationPlugin for ExampleTransformationPlugin {
    fn name(&self) -> &str {
        "Example Transformation"
    }

    fn transform(&self, instructions: &mut Vec<u8>) {
        // Example transformation: Increment each byte
        for i in 0..instructions.len() {
            instructions[i] = instructions[i].wrapping_add(1);
        }
    }
}

// const TIMEOUT: u64 = 30;
static LOGGER: utils::Logger = utils::Logger;

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
    let opts = match options::parse_options() {
        Ok(o) => o,
        Err(e) => {
            error!("{e}");
            return;
        }
    };

    print_banner();
    options::print_summary(&opts);

    if opts.arch.to_lowercase() != "x86" && opts.arch.to_lowercase() != "arm" {
        error!("Currently only x86 and ARM architectures are supported.");
        return;
    }

    if opts.freq > 0.8 || opts.cycle > 2 {
        warn!("Deoptimization parameters are too aggressive!");
        warn!("The output size will drastically increase.")
    }

    let file = match utils::read_file(opts.file.clone()) {
        Ok(f) => f,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    let mut out_file = match File::create(opts.outfile.clone()) {
        Ok(f) => f,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("Input file size: {}", file.len());
    let mut deopt = match opts.arch.to_lowercase().as_str() {
        "x86" => x86_64::Deoptimizer::new(),
        "arm" => arm::Deoptimizer::new(),
        _ => return,
    };
    deopt.freq = opts.freq;
    deopt.allow_invalid = opts.allow_invalid;
    deopt.set_skipped_offsets(opts.skip_offsets);
    if let Err(e) = deopt.set_transform_gadgets(opts.transforms) {
        error!("{}", e);
        return;
    }
    if let Err(e) = deopt.set_syntax(opts.syntax) {
        error!("{}", e);
        return;
    }
    let start_addr = match u64::from_str_radix(opts.addr.trim_start_matches("0x"), 16) {
        Ok(addr) => addr,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let mut input = file.clone();
    let mut output = Vec::new();

    // Initialize the plugin manager and load example plugins
    let mut plugin_manager = PluginManager::new();
    plugin_manager.load_transformation_plugin(Box::new(ExampleTransformationPlugin));
    example_plugins::load_example_plugins(&mut plugin_manager);

    // Initialize the visualization tool
    let visualization_tool = VisualizationTool::new();

    // Initialize the reporting module
    let reporting_module = ReportingModule::new();
    let mut applied_transformations = Vec::new();

    for cycle in 0..opts.cycle {
        info!("Analyzing input binary...");
        if let Err(e) = deopt.analyze(&input) {
            error!("{}", e);
            return;
        }
        info!("Transforming instructions...");
        if let Err(e) = deopt.transform(&mut input) {
            error!("{}", e);
            return;
        }
        // Apply plugins
        plugin_manager.apply_transformations(&mut input);
        applied_transformations.push("Example Transformation".to_string());
        info!("Encoding transformed instructions...");
        output = match deopt.encode(&input) {
            Ok(b) => b,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };

        // Visualize the transformed instructions
        let visualization_filename = format!("{}_cycle_{}.txt", opts.outfile, cycle);
        if let Err(e) = visualization_tool.visualize(&output, &visualization_filename) {
            error!("Failed to generate visualization: {}", e);
        }
        
        // Generate report of applied transformations
        let report_filename = format!("{}_report_{}.txt", opts.outfile, cycle);
        if let Err(e) = reporting_module.generate_report(&applied_transformations, &input, &output, &report_filename) {
            error!("Failed to generate report: {}", e);
        }

        input = output.clone();
    }

    if opts.source.is_empty() {
        match out_file.write_all(&output) {
            Ok(()) => (),
            Err(e) => {
                error!("{}", e);
                return;
            }
        }
        info!("De-optimized binary written into {}", opts.outfile);
    } else {
        let source = match deopt.disassemble(opts.bitness, start_addr, output) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        match out_file.write_all(source.as_bytes()) {
            Ok(()) => (),
            Err(e) => {
                error!("{}", e);
                return;
            }
        }
        info!("De-optimized assembly source written into {}", opts.outfile);
    }

    println!("{} All done!", "[✔]".green().bold());
}

fn print_banner() {
    let banner_b64 = b"ICBfX19fXyAgICAgICAgICAgICAgIF9fX18gICAgICAgIF8gICAgICAgICAgICAgICAgICAgICAgICAgICAgCiB8ICBfXyBcICAgICAgICAgICAgIC8gX18gXCAgICAgIHwgfCAo4piiKSAgICAgICAgICjimKIpICAgICAgICAgICAgIAogfCB8ICB8IHwgX19fIF9fX19fX3wgfCAgfCB8XyBfXyB8IHxfIF8gXyBfXyBfX18gIF8gX19fX19fXyBfIF9fIAogfCB8ICB8IHwvIF8gXF9fX19fX3wgfCAgfCB8ICdfIFx8IF9ffCB8ICdfIGAgXyBcfCB8XyAgLyBfIFwgJ19ffAogfCB8X198IHwgIF9fLyAgICAgIHwgfF9ffCB8IHxfKSB8IHxffCB8IHwgfCB8IHwgfCB8LyAvICBfXy8gfCAgIAogfF9fX19fLyBcX19ffCAgICAgICBcX19fXy98IC5fXy...

    println!(
        "{}",
        String::from_utf8(
            BASE64_STANDARD
                .decode(banner_b64)
                .expect("base64 decode failed!")
        )
        .expect("can not convert to utf8")
    );
}
