use std::path::Path;
use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PrayArgs {
    /// the subcommand to use
    #[clap(subcommand)]
    pub subcommand: CmdType,
    #[arg(short = 'g', long = "gui")]
    pub use_gui: bool,
}

#[derive(Debug, Subcommand)]
pub enum CmdType {
    /// start the map editor and make a world
    AWorld(MapEditCmd),
    /// read in a config file and make the next set of codegen files
    Creation(ReadBabbleCmd),
    /// read in a file and do a dummy run
    Illumination(ReadBabbleCmd),
}

#[derive(Debug, Args)]
pub struct ReadBabbleCmd {
    /// path to file
    pub file_path: String,
    pub build_objects: bool,

}

#[derive(Debug, Args)]
pub struct MapEditCmd {
    pub map_name: String,
    pub file_path: String,
}

