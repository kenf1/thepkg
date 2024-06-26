//encrypt & decrypt logic
pub fn cryptfn(task: &str,input: &str,ref_dict: &str,shift: usize) -> String{
    let mut charvec: Vec<char> = Vec::new();

    for c in input.chars(){
        let index = ref_dict.find(c);
        match index {
            Some(v) => {
                //encrypt vs decrypt
                match task {
                    "encrypt" => {
                        //get shift index (wraps to beginning if overflow)
                        let new_index = (v + shift) % ref_dict.len();

                        //get char from new index -> append to vec
                        if let Some(new_char) = ref_dict.chars().nth(new_index){
                            charvec.push(new_char);
                        }
                    },
                    "decrypt" => {
                        //get shift index (wraps to beginning if overflow)
                        let new_index = (v - shift) % ref_dict.len();

                        //get char from new index -> append to vec
                        if let Some(new_char) = ref_dict.chars().nth(new_index){
                            charvec.push(new_char);
                        }
                    },
                    _ => {
                        println!("Options are: encrypt or decrypt");
                        return String::from("Error");
                    }
                }
            },
            None => {
                //break when encounter char not in ref
                println!("Error: entered string contains characters not found in reference dict.");
                return String::from("Error");
            },
        }
    }

    vec_to_str(charvec)
}

//convert Vec<char> -> String
fn vec_to_str(input_vec: Vec<char>) -> String{
    let output_str: String = input_vec
        .iter()
        .collect();

    output_str
}