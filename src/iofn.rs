use std::error::Error;
use std::fs;
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