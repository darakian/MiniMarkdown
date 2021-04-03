#[cfg(test)]
mod tests {
    use crate::{lex};
    
    #[test]
    fn test_lex() {
        let heading_tests = vec![
            ("# Heading level 1", "<h1>Heading level 1</h1>"),
            ("## Heading level 2", "<h2>Heading level 2</h2>"),
            ("### Heading level 3", "<h3>Heading level 3</h3>"),
            ("#### Heading level 4", "<h4>Heading level 4</h4>"),
            ("##### Heading level 5", "<h5>Heading level 5</h5>"),
            ("###### Heading level 6", "<h6>Heading level 6</h6>"),
            ("####### Invalid Heading level 7", "<h6>Heading level 6</h6>"),
        ];
        for test in heading_tests.iter(){
            println!("Testing: {} -> {}", test.0, test.1);
            lex(test.0);
        }

        let bold_tests = vec![
            ("I just love **bold text**.", "I just love <strong>bold text</strong>."),
        ];
        for test in bold_tests.iter(){
            println!("Testing bold: {} -> {}", test.0, test.1);
            lex(test.0);
        }
    }
}

#[derive(Debug)]
enum MarkdownToken{
    MarkdownPlaintext(String),
    MarkdownBeginHeader(u8),
}

#[derive(Debug)]
struct MarkdownParseError{
    reason: String,
}


/*
Tokens
#, ##, ... (headers)
***x*** / ___x___
**x** / __x__
*x* / _x_
>, >>, ...
newline (two or more spaces at end of line)
1., 2.,  (number dot ordered lists. Can nest)
-/+  (unordered lists. Can nest)
code blocks (4 spaces for block. Can be multiline. Can be in list (8 spaces if so))
images ![text](link)
`x` code
``x`` escape backticks in x
links [text](link) / [text](link "hover text")
link without text <link>

*/

fn lex(source: &str) -> (){
    let mut char_iter = source.trim().chars().peekable();
    let mut tokens = Vec::new();
    while char_iter.peek().is_some(){
        match char_iter.peek(){
            None => {return},
            Some('#') => {
                let heading = lex_heading(&mut char_iter);
                match heading {
                    Ok(h) => tokens.push(h),
                    Err(e) => println!("{:?}", e),
                }
            }
            Some(_) => {
                let c = char_iter.next().unwrap();
                match tokens.last_mut() {
                    Some(markdown_token) => {
                        match markdown_token {
                            MarkdownToken::MarkdownPlaintext(mp) => mp.push(c),
                            _ => tokens.push(MarkdownToken::MarkdownPlaintext(c.to_string())),
                        }
                    }
                    None => tokens.push(MarkdownToken::MarkdownPlaintext(c.to_string())),
                }
            },
        }
        println!("Token: {:?}", tokens.last());
    }
}


use std::cmp;
fn lex_heading(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<MarkdownToken, MarkdownParseError>{
    let mut hashes = 0;
    while char_iter.peek() == Some(&'#'){
        hashes+=1;
        char_iter.next();
    }
    match char_iter.peek(){
        Some(' ') => {
            char_iter.next();
            return Ok(MarkdownToken::MarkdownBeginHeader(cmp::min(6, hashes)));
        },
        _ => {Err(MarkdownParseError{reason: "No space after final #".to_string()})}
    }
}