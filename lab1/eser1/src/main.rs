use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
}

fn slugify(s: &str) -> String {
    let mut counter = 0;
    let mut s0 = String::new();
    for chara in s.chars() {
        let mut return_char = conv(chara);
        if return_char == '-' {
            counter += 1;
        } else { counter = 0; }
        if counter < 2 { s0.push(return_char); }
    }
    return s0;
}

fn conv(c: char) -> char {
    let mut s = String::from(c);
    s = s.to_lowercase();
    let char_vec: Vec<char> = s.chars().collect();
    let regex = Regex::new(r"[a-z0-9]").unwrap();
    let a = regex.is_match(&s);
    if a { c } else {
        let mut flag = false;
        let c1 = char_vec[0];
        const SUBS_I: &str =
            "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
        const SUBS_O: &str =
            "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
        let mut counter = 0;
        let mut counter2 = 0;
        for chara in SUBS_I.chars() {
            if c1 == chara {
                break;
            }
            counter += 1;
        }
        let mut final_char = c1;
        for chara in SUBS_O.chars() {
            if counter == counter2 {
                final_char = chara;
                flag = true;
            }
            counter2 += 1;
        }
        if flag { final_char } else { '-' }
    }
}

fn main() {
    let arg= Args::parse();
    let s1 = slugify(&arg.slug_in);
    println!("{}", s1);
}

#[cfg(test)]
mod tests {
    use super::*;
}
#[test]
fn my_first_test() {
    // valore = preparazione test
    assert_eq!(1, 1)
}

