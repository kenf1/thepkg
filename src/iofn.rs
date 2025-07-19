use csv::Writer;
use dotenvy::dotenv;
use polars::prelude::*;
use std::{
    env,
    error::Error,
    fs::{self, ReadDir},
};

//confirm file exists
pub fn file_exists(full_path: &str) -> Result<(), Box<dyn Error>> {
    //file DNE
    if fs::metadata(full_path).is_err() {
        eprintln!("Error: file not found");
        return Err("File not found".into());
    }

    Ok(())
}

//save dataframe (vector of String vectors) as csv
pub fn save_csv(
    df: Vec<Vec<String>>,
    folder_name: &str,
    file_name: &str,
) -> Result<(), Box<dyn Error>> {
    //create folders if DNE
    fs::create_dir_all(folder_name)?;

    //write to file
    let mut wtr = Writer::from_path(format!("{folder_name}/{file_name}.csv"))?;
    for vec in df {
        wtr.write_record(vec)?;
    }
    wtr.flush()?;

    Ok(())
}

//save polars DataFrame as csv
pub fn save_df(
    mut df: DataFrame,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    // create file
    let mut file = fs::File::create(output_path)?;
    CsvWriter::new(&mut file).finish(&mut df)?;

    println!("{output_path} saved");
    Ok(())
}

//store results as Vec<String>
pub fn path_tree(paths: ReadDir, debug_print: bool) -> Vec<String> {
    let mut path_vec: Vec<String> = Vec::new();

    for path in paths.flatten() {
        let path_str = path.path().display().to_string();
        path_vec.push(path_str);
    }

    if debug_print {
        println!("{path_vec:?}\n---\n");
    }

    path_vec
}

// filter path: Vec<String>, not destructive
pub fn paths_filter(
    paths: Vec<String>,
    excluded: Vec<&str>,
    debug_print: bool,
) -> Vec<String> {
    let filt_paths: Vec<String> = paths
        .iter()
        .filter(|&element| {
            !excluded
                .iter()
                .any(|&exclusion| element.contains(exclusion))
        })
        .cloned()
        .collect();

    if debug_print {
        println!("Paths to remove target folder from:\n{filt_paths:?}\n");
    }

    filt_paths
}

// rm all target folders
pub fn loop_rm(paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    for item in paths {
        let result = fs::remove_dir_all(format!("{item}/target"));

        // output outcome message
        match result {
            Ok(_) => {
                println!("{item}/target removed successfully.")
            }
            Err(e) => eprintln!("Error: {e} -> Skipped"),
        }
    }

    println!("\nComplete");
    Ok(())
}

/*
    wrapper function to rm all target folders from target_path
    params:
        excluded = Vec<str> to exclude from all found paths
        print_allpaths = prints all subpaths if true
        print_filtpaths = print all subpaths (after filter) if true
*/
pub fn rm_alltarget(
    target_path: &str,
    excluded: Vec<&str>,
    print_allpaths: bool,
    print_filtpaths: bool,
) -> Result<(), Box<dyn Error>> {
    //store all paths
    let origpath = fs::read_dir(target_path)?;
    let all_paths = path_tree(origpath, print_allpaths);

    //rm misc paths (rm_target project excluded)
    let filt_paths = paths_filter(all_paths, excluded, print_filtpaths);

    //rm target folder for each path in filt_paths
    loop_rm(filt_paths)?;

    Ok(())
}

//import item from .env file
pub fn import_env(item: &str) -> String {
    dotenv().ok();

    env::var(item).expect("Missing env var")
}
