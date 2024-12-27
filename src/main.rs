use std::io::stdin;
use dialoguer::Select;
 
fn main() {

    let options = vec![
        "Eng to Ar",
        "Ar to Eng"
    ];

    let selection = Select::new()
        .with_prompt("Please select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => {
            let mut text_to_change = String::new();
            stdin().read_line(&mut text_to_change).unwrap();

            let mut changed_text = String::new();
            for char in text_to_change.chars() {
                let letter = en_to_ar_letter(char.to_string());
                changed_text = changed_text + letter.as_str();
            }

            println!("Your text translates to: \n - {}", &changed_text)
        }
        1 => {
            let mut text_to_change = String::new();
            stdin().read_line(&mut text_to_change).unwrap();

            let mut changed_text = String::new();
            for char in text_to_change.chars() {
                let letter = ar_to_en_letter(char.to_string());
                changed_text = changed_text + letter.as_str();
            }

            println!("Your text translates to: \n - {}", &changed_text)
        }
        _ => {
            println!("ERROR: invalid selection");
        }
    }
    let test = ar_to_en_letter(("ت").to_string());
    println!("{}", test)
}


struct KBLayout {
    ar: Vec<&'static str>,
    en: Vec <&'static str>
}

fn ar_to_en_letter(letter: String) -> String {
    let kb_layout = KBLayout {
        ar: vec!["ذ" , "ض", "ص", "ث", "ق", "ف", "غ", "ع", "ه", "خ", "ح", "ج", "د", "ش", "س", "ي", "ب", "ل", "ا", "ت", "ن", "م", "ك", "ط", "ئ", "ء", "ؤ", "ر", "لا", "ى", "ة", "و", "ز", "ظ"],
        en: vec!["`" , "q", "w", "e", "r", "t", "y", "u", "i", "o", "p","[" ,"]" , "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/"]
        };   
    let ara = kb_layout.ar; 
    let eng = kb_layout.en;
    for (i, &ar_letter) in ara.iter().enumerate() {
        if ar_letter == letter {
            return eng[i].to_string();
        }
    }
    return letter;
}

fn en_to_ar_letter(letter: String) -> String {
    let kb_layout = KBLayout {
        ar: vec!["ذ" , "ض", "ص", "ث", "ق", "ف", "غ", "ع", "ه", "خ", "ح", "ج", "د", "ش", "س", "ي", "ب", "ل", "ا", "ت", "ن", "م", "ك", "ط", "ئ", "ء", "ؤ", "ر", "لا", "ى", "ة", "و", "ز", "ظ"],
        en: vec!["`" , "q", "w", "e", "r", "t", "y", "u", "i", "o", "p","[" ,"]" , "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/"]
        };   
    let ara = kb_layout.ar; 
    let eng = kb_layout.en;
    for (i, &eng_letter) in eng.iter().enumerate() {
        if eng_letter == letter {
            return ara[i].to_string();
        }
    }
    return letter;
}


