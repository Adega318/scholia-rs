use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Structure-aware chunker for academic documents")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Split a document into chunks with metadata
    Chunk(ChunkArgs),
    /// Show the structure of a document without chunking it
    Inspect(InspectArgs),
}

#[derive(Args)]
pub(crate) struct ChunkArgs {
    /// Input file, or `-` for stdin
    pub(crate) input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Input format (default: detected from the file extension)
    #[arg(long = "from", value_enum, default_value_t = Kind::Auto)]
    from: Kind,

    /// Chunking strategy
    #[arg(long, value_enum, default_value_t = Strategy::Section)]
    strategy: Strategy,

    /// Maximum tokens per chunk
    #[arg(long, default_value_t = 512)]
    max_tokens: usize,

    /// Tokens repeated between consecutive chunks of the same section
    #[arg(long, default_value_t = 64)]
    overlap: usize,

    /// Output format
    #[arg(long, value_enum, default_value_t = Format::Jsonl)]
    format: Format,
}

#[derive(Args)]
pub(crate) struct InspectArgs {
    /// Input file, or `-` for stdin
    pub(crate) input: PathBuf,

    /// Input format (default: detected from the file extension)
    #[arg(long = "from", value_enum, default_value_t = Kind::Auto)]
    from: Kind,

    /// Print the outline as JSON instead of a tree
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum Kind {
    Auto,
    Md,
    Latex,
    Pdf,
    Txt,
}

#[derive(Clone, Copy, ValueEnum)]
enum Strategy {
    /// Respect section boundaries; split oversized sections by token budget
    Section,
    /// Ignore structure; split the whole document by token budget
    Tokens,
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Json,
    Jsonl,
}
