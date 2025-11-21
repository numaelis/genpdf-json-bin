use clap::Parser;
use std::env;
use genpdf_json;

#[derive(Parser, Debug)]
#[command(version, about = "Fast PDF generator from JSON data", long_about = None)]
struct Args {
    /// The file contains the data in JSON format. 
    #[arg(short, long)]
    json_file: Option<String>,
    /// the sqlite .db file.
    #[arg(short, long)]
    sqlite_file: Option<String>,
    /// the name of the output .pdf file (include the path)
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<_> = env::args().skip(1).collect();
    
    //simple support
    if args.len() == 2 && args[0].contains("--") == false && args[1].contains("--") == false {
        let json_file = &args[0];
        let output_file = &args[1];
        if json_file.contains(".db"){
            let _ = genpdf_json::render_file_from_sqlite(json_file, output_file)?;    
        }else{
            let _ = genpdf_json::render_json_file(json_file, output_file)?;    
        } 
    }else{
        if args.len() == 1 {
            if args[0].contains("--help") || args[0].contains("-h"){
                println!("");
                println!("<using genpdf-json version {:?}>", genpdf_json::VERSION);
                println!("");
            }
        }
        // clap
        let args = Args::parse();        
        if let Some(json_file) = args.json_file.as_deref(){
            let _ = genpdf_json::render_json_file(json_file, args.output)?; 
        }else{
            if let Some(sqlite_file) = args.sqlite_file.as_deref(){
                let _ = genpdf_json::render_file_from_sqlite(sqlite_file, args.output)?;
            }else{
                panic!("No json or output file argument, run --help");
            }
        }        
    }      

    Ok(())
}
