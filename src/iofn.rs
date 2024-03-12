use std::error::Error;
use std::fs::{self,ReadDir};
use csv::Writer;
use polars::prelude::*;

//save dataframe (vector of String vectors) as csv
pub fn save_csv(df: Vec<Vec<String>>,folder_name: &str,file_name: &str) -> Result<(),Box<dyn Error>>{
    //create folders if DNE
    fs::create_dir_all(folder_name)
        .expect("Unable to create file.");

    //write to file
    let mut wtr = Writer::from_path(format!("{}/{}.csv",folder_name,file_name))?;
    for vec in df{
        wtr.write_record(vec)?;
    }
    wtr.flush()?;

    Ok(())
}

//save polars DataFrame as csv
pub fn save_df(mut df: DataFrame,output_path: &str){
    //create file
    let mut file = fs::File::create(output_path)
        .unwrap();

    CsvWriter::new(&mut file)
        .finish(&mut df)
        .unwrap();

    println!("{} saved",output_path);
}

//store results as Vec<String>
pub fn path_tree(paths: ReadDir,debug_print: bool) -> Vec<String>{
    let mut path_vec: Vec<String> = Vec::new();

    for path in paths{
        if let Ok(temp) = path{
            let path_str = temp
                .path()
                .display()
                .to_string();
            path_vec.push(path_str);
        }
    }

    if debug_print == true{
        println!("{:?}\n---\n",path_vec);
    }

    path_vec
}

//filter path: Vec<String>, not destructive
pub fn paths_filter(paths : Vec<String>,excluded: Vec<&str>,debug_print: bool) -> Vec<String>{
    let filt_paths: Vec<String> = paths
        .iter()
        .filter(|&element| !excluded.iter().any(|&exclusion| element.contains(exclusion)))
        .cloned()
        .collect();

    if debug_print == true{
        println!("Paths to remove target folder from:\n{:?}\n",filt_paths);
    }

    filt_paths
}

//rm all target folders
pub fn loop_rm(paths: Vec<String>){
    for item in paths{
        let result = fs::remove_dir_all(
            format!("{}/target",item.to_string())
        );

        //output outcome message
        match result{
            Ok(_) => println!("{}/target removed successfully.",item.to_string()),
            Err(e) => eprintln!("Error: {} -> Skipped",e),
        }
    }

    println!("\nComplete");
}

/*
    wrapper function to rm all target folders from target_path
    params:
        excluded = Vec<str> to exclude from all found paths
        print_allpaths = prints all subpaths if true
        print_filtpaths = print all subpaths (after filter) if true
*/
pub fn rm_alltarget(target_path: &str,excluded: Vec<&str>,print_allpaths: bool,print_filtpaths: bool){
    //store all paths
    let origpath = fs::read_dir(target_path)
        .unwrap();
    let all_paths = path_tree(origpath,print_allpaths);

    //rm misc paths (rm_target project excluded)
    let filt_paths = paths_filter(all_paths,excluded,print_filtpaths);

    //rm target folder for each path in filt_paths
    loop_rm(filt_paths);
}