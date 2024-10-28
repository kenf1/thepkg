//encrypt & decrypt logic
pub fn cryptfn(task: &str,input: &str,ref_dict: &str,shift: usize) -> 
Result<String, String>{
    let mut charvec: Vec<char> = Vec::new();

    for c in input.chars(){
        match ref_dict.find(c){
            Some(v) => {
                let new_index = match task {
                    //encrypt vs decrypt
                    "encrypt" => (v + shift) % ref_dict.len(),
                    "decrypt" => (v - shift + ref_dict.len()) % ref_dict.len(), // Ensure non-negative index
                    _ => {
                        //unaccepted input
                        return Err(String::from("Options are: encrypt or decrypt"));
                    }
                };

                //get char from new index -> append to vec
                if let Some(new_char) = ref_dict.chars().nth(new_index){
                    charvec.push(new_char);
                } else {
                    return Err(String::from("Error: reference dictionary is invalid."));
                }
            },
            None => {
                //break when encounter char not in ref
                return Err(String::from("Error: entered string contains characters not found in reference dict."));
            },
        }
    }

    Ok(vec_to_str(charvec))
}

//convert Vec<char> -> String
fn vec_to_str(input_vec: Vec<char>) -> String{
    //implicit return
    input_vec
        .iter()
        .collect()
}