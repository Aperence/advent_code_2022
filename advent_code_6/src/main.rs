use std::fs;

fn no_dup(s : &str) -> bool{
    for offset in 1..14{
        if s[..offset].contains(&s[(offset as usize)..(offset+1 as usize)]){
            return false;
        }
    }
    true
}

fn main() {
    let str = fs::read_to_string("test.txt").unwrap();

    let mut index = 0;

    for i in 13..str.len(){
        if no_dup(&str[i-13..i+1]){
            index = i + 1;
            break;
        }
    }

    println!("{index}")
}   
