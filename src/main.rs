use std::env;

// Human readble cow format
// replace eye with 'e', thoughts with 't' and tongue with 'y'
// TODO: probably better to use somekind of unicode characters
const COW_SAY: &str = r#"
        t   ^__^
         t  (ee)\_______
            (__)\       )\/\
             y  ||----w |
                ||     ||

"#;

const MAX_LENGTH: u16 = 40; // maximum text length

struct CowParts {
    pub eyes: String,
    pub tongue: String,
    pub thoughts: String,
}

impl Default for CowParts {
    fn default() -> Self {
        Self {
            eyes: "oo".into(),
            tongue: " ".into(),
            thoughts: "\\".into(),
        }
    }
}

pub fn construct_baloon(message: &str) -> String {
    // Construct text baloon.
    if message.len() < MAX_LENGTH as usize {
        let top = format!(" {:_^width$}", "_", width = message.len());
        let bottom = format!(" {:-^width$}", "-", width = message.len());

        return format!("{}\n<{}>\n{}", top, message, bottom);
    } else {
        let mut lines: Vec<String> = vec![];
        let mut current_line: Vec<&str> = vec![];
        let mut cl_len: usize = 0; // current line length

        for word in message.split(" ") {
            println!("{}, {:?}", word, lines);
            // +1 due to space between words
            if cl_len + word.len() + 1 < MAX_LENGTH as usize {
                cl_len += word.len() + 1;
                current_line.push(&word);
            } else {
                lines.push(format!(
                    "({:^1$})",
                    current_line.join(" "),
                    MAX_LENGTH as usize
                ));
                current_line.clear();

                cl_len = word.len() + 1;
                current_line.push(&word);
            }
        }
        let top = format!(" {:_^width$}", "_", width = MAX_LENGTH as usize);
        let bottom = format!(" {:-^width$}", "-", width = MAX_LENGTH as usize);

        return format!("{}\n{}\n{}", top, lines.join("\n"), bottom);
    }
}

fn main() {
    // parse command line arguments
    // first argument if available determinate CowType
    match env::args().len() {
        2 => {
            // only one argument, assume it's text
            let text = env::args().nth(1).expect("Can't parse text argument.");
            println!("{}", COW_SAY.to_string().replace("$thoughts", &text));
        }
        3 => {
            let flag = env::args().nth(1).expect("Can't parse option argument.");
            let text = env::args().nth(2).expect("Can't pasrse text argument");
            let cow_parts: CowParts = match flag.as_str() {
                "-b" => CowParts {
                    eyes: "==".into(),
                    ..Default::default()
                },
                "-d" => CowParts {
                    eyes: "xx".into(),
                    tongue: "U".into(),
                    ..Default::default()
                },
                "-g" => CowParts {
                    eyes: "$$".into(),
                    ..Default::default()
                },
                "-p" => CowParts {
                    eyes: "@@".into(),
                    ..Default::default()
                },
                "-s" => CowParts {
                    eyes: "**".into(),
                    tongue: "U".into(),
                    ..Default::default()
                },
                "-t" => CowParts {
                    eyes: "--".into(),
                    ..Default::default()
                },
                "-w" => CowParts {
                    eyes: "OO".into(),
                    ..Default::default()
                },
                "-y" => CowParts {
                    eyes: "..".into(),
                    ..Default::default()
                },
                _ => panic!("Wrong option value."),
            };
            println!(
                "{}{}",
                construct_baloon(text.as_str()),
                COW_SAY
                    .to_string()
                    .replace("t", &cow_parts.thoughts)
                    .replace("ee", &cow_parts.eyes)
                    .replace("y", &cow_parts.tongue)
            );
        }
        _ => panic!("wrong arguments!"),
    }
}
