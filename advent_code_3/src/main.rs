use std::fs;

fn get_dup(s1 : &str, s2 : &str) -> char{
    for c in s1.chars(){
        if s2.contains(c){
            return c;
        }
    }
    'z'
}

fn get_dup_3(s1 : &str, s2 : &str, s3 : &str) -> char{
    for c in s1.chars(){
        if s2.contains(c) && s3.contains(c){
            return c;
        }
    }
    'z'
}

fn priority(c : char) -> u32{
    if 'a' <= c && c <= 'z'{
        return (c as u32) - ('a' as u32) + 1;
    }else if 'A' <= c && c <= 'Z'{
        return (c as u32) - ('A' as u32) + 27;
    }else{
        return 0;
    }
}


fn main() {
    let s = fs::read_to_string("test.txt").unwrap();

    let mut score = 0;

    let mut v : Vec<&str> = vec![];

    for line in s.lines(){
        /* 
        let first_batch = &line[..line.len()/2];
        let second_batch = &line[line.len()/2..];

        let dup = get_dup(first_batch, second_batch);     
        score += priority(dup);*/

        v.push(line);

        if v.len() == 3{
            let dup = get_dup_3(v.pop().unwrap(), v.pop().unwrap(), v.pop().unwrap());
            score += priority(dup);
        }
    }

    println!("{score}");
}
