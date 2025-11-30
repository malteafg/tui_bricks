use clap::Parser;

fn main() {
    let args = rebrickable_client::Args::parse();
    rebrickable_client::run(args);
}
