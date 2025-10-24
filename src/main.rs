
use std::env;
use genpdf_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Missing argument: json file or output file");
    }
    
    let json_file = &args[0];
    let output_file = &args[1];    
    
    if json_file.contains(".db"){
        let _ = genpdf_json::render_file_from_sqlite(json_file, output_file)?;    
    }else{
        let _ = genpdf_json::render_json_file(json_file, output_file)?;    
    }    

    Ok(())
}
