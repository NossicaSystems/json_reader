use std::env;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq)]
enum ParserStage {
    LookingForOpeningBrace,
    LookingForKeyOpenQuote,
    LookingForKeyCloseQuote,
    LookingForValueOpenQuote,
    LookingForValueCloseQuote,
    LookingForColon,
    LookingForEndBraceOrComma,
    FoundEndBrace,
    Error,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].as_str();

    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let trim_s = s.replace(" ", "").replace("\n", "");
    println!("{}", trim_s);

    if trim_s.len() < 2 {
        println!("0");
        return;
    }

    if trim_s.starts_with("{") == false {
        println!("0");
        return;
    }

    if trim_s.ends_with("}") == false {
        println!("0");
        return;
    }

    let json_script: Vec<char> = trim_s.chars().collect();
    let mut state = ParserStage::LookingForOpeningBrace;
    let mut i = 0;
    while i < json_script.len() {
        println!("current char is {} i is {}", json_script[i], i);
        match state {
            ParserStage::LookingForOpeningBrace => match json_script[i] {
                '{' => {
                    state = ParserStage::LookingForKeyOpenQuote;
                    i = i + 1;
                    continue;
                }
                _ => {
                    state = ParserStage::Error;
                    break;
                }
            },
            ParserStage::LookingForKeyOpenQuote => match json_script[i] {
                '"' => {
                    state = ParserStage::LookingForKeyCloseQuote;
                    i = i + 1;
                    continue;
                }
                _ => {
                    state = ParserStage::Error;
                    break;
                }
            },
            ParserStage::LookingForKeyCloseQuote => match json_script[i] {
                '"' => {
                    state = ParserStage::LookingForColon;
                    i = i + 1;
                    continue;
                }
                _ => {
                    i = i + 1;
                    continue;
                }
            },
            ParserStage::LookingForColon => match json_script[i] {
                ':' => {
                    state = ParserStage::LookingForValueOpenQuote;
                    i = i + 1;
                    continue;
                }
                _ => {
                    state = ParserStage::Error;
                    break;
                }
            },
            ParserStage::LookingForValueOpenQuote => match json_script[i] {
                '[' => {
                    // once we read in a type we can only read that type in until we find the matching ]
                }
                '{' => {}
                '"' => {
                    state = ParserStage::LookingForValueCloseQuote;
                    i = i + 1;
                    continue;
                }
                c if c.is_numeric() => {
                    while json_script[i].is_numeric() {
                        i = i + 1;
                    }
                    state = ParserStage::LookingForEndBraceOrComma;
                    continue;
                }
                'f' => {
                    let mut bool_word = String::new();

                    for mut j in 0..5 {
                        bool_word.push(json_script[i]);
                        i = i + 1;
                    }

                    if bool_word == "false" {
                        state = ParserStage::LookingForEndBraceOrComma;
                    } else {
                        state = ParserStage::Error;
                        break;
                    }
                    continue;
                }
                't' => {
                    let mut bool_word = String::new();
                    for mut j in 0..4 {
                        bool_word.push(json_script[i]);
                        i = i + 1;
                        println!("i is {}", i);
                        //                      j = j + 1;
                    }

                    if bool_word == "true" {
                        state = ParserStage::LookingForEndBraceOrComma
                    //      i = i + 4;
                    } else {
                        state = ParserStage::Error;
                        break;
                    }
                    continue;
                }
                'n' => {
                    let mut bool_word = String::new();
                    for mut j in 0..4 {
                        bool_word.push(json_script[i]);
                        i = i + 1;
                        println!("i is {}", i);
                        //                      j = j + 1;
                    }

                    if bool_word == "null" {
                        state = ParserStage::LookingForEndBraceOrComma
                    //      i = i + 4;
                    } else {
                        state = ParserStage::Error;
                        break;
                    }
                    continue;
                }
                _ => {
                    state = ParserStage::Error;
                    break;
                }
            },
            ParserStage::LookingForValueCloseQuote => match json_script[i] {
                '"' => {
                    state = ParserStage::LookingForEndBraceOrComma;
                    i = i + 1;
                    continue;
                }
                _ => {
                    i = i + 1;
                    continue;
                }
            },
            ParserStage::LookingForEndBraceOrComma => match json_script[i] {
                '}' => {
                    state = ParserStage::FoundEndBrace;
                    break;
                }
                ',' => {
                    state = ParserStage::LookingForKeyOpenQuote;
                    i = i + 1;
                    continue;
                }
                _ => {
                    state = ParserStage::Error;
                    break;
                }
            },
            _ => {
                break;
            }
        }
    }
    if state == ParserStage::FoundEndBrace {
        println!("1");
    } else {
        println!("0");
    }
}
