pub fn translate_phrase (phrase: &String) -> String {
    // create vector from given String
    let arr: Vec<&str> = phrase.as_str().split(" ").collect();
    // get length of vector
    let size: usize = arr.len();

    // for use in loop
    // beginning 
    let mut start = 0;
    let mut final_translation = "".to_string();

    loop {
        
        if start >= size {
            // this means nothing was found
            break;
        }

        // hi how are you
        for i in start..= size-1 {

            let mut phrase_found = false;
            let mut phrase = "".to_string();

            // how are you (start = 1)
            // how are you
            for k in start..=i {
                phrase.push_str(&arr[k]);

                let try_translate = translate_word(&phrase.to_string());

                if try_translate != "?" {
                    
                    if final_translation.len() != 0 {
                        final_translation.push_str(" ");
                    }
                    
                    final_translation.push_str(&try_translate.as_str());
                    start = k + 1;
                    phrase_found = true;
                    break;
                }
                // is a "?" from translate_word
                else {
                    phrase.push_str(" ");
                }
            }
            
            if phrase_found == true {
                break;
            }
            else if i == size - 1 {
                start = start + 1;
            }
        }
        
    }   
    
    if final_translation.len() == 0 {
        "??".to_string()
    }
    else {
        final_translation
    }

}


pub fn translate_word (s: &String) -> String {

    match s.to_ascii_lowercase().as_str(){
            
        "hi" | "hello" | "howdy" => "hola".to_string(),
        "bye" | "goodbye" | "farwell" => "adiós".to_string(),
        "how are you" => "cómo estás".to_string(),
        "good morning" => "buenos días".to_string(),
        "good afternoon" => "buenas tardes".to_string(),
        "good evening" | "good night" => "benas noches".to_string(),
        "see you later" => "hasta luego".to_string(),
        
        _ => "?".to_string(),
        
    }

}
