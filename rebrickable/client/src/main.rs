use clap::Parser;

fn main() {
    let args = rebrickable_client::cli::Args::parse();
    rebrickable_client::run(args);
}
