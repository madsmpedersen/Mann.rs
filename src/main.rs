use clap::Parser;
use serde::{Deserialize, Serialize};
use toml::Value;
#[derive(Serialize, Deserialize, Debug)]
struct MannParams {
    fn_u: std::path::PathBuf,
    fn_v: std::path::PathBuf,
    fn_w: std::path::PathBuf,
    Nx: u64,
    Ny: u64,
    Nz: u64,
    Lx: f64,
    Ly: f64,
    Lz: f64,
    ae: f64,
    L: f64,
    gamma: f64,
    seed: u64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Mann turbulence generator.
struct Args {
    /// Input file paths
    input: Vec<std::path::PathBuf>,


}

fn main() {
    let args = Args::parse();

    for filename in args.input.iter() {
        println!("Hello {:?}! {}", filename, filename.exists());

        let contents = std::fs::read_to_string(filename).unwrap();
        println!("Hello {}", contents);
        let val: Value = contents.parse().unwrap();
        
        println!("Hello {:?}", val);
        //   let mann: MannParams = toml::from_str(&contents).unwrap();
        let mann: MannParams = val["windfields"]["Mann"].to_owned().try_into().unwrap();
        println!("Hello {:?}", mann);


        println!("{}", toml::to_string(&mann).unwrap());

        

    }
}
