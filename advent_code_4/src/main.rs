use std::fs;

fn is_contained(lo1 : i64, hi1 : i64, lo2 : i64, hi2 : i64) -> bool{
    if lo1 <= lo2 && hi1 >= hi2{
        true
    }else if lo2 <= lo1 && hi2 >= hi1{
        true
    }else if lo2 <= hi1 && hi2 >= lo1{
        true
    }else if lo1 <= hi2 && hi1 >= lo2{
        true
    }else{
        false
    }
}

fn main() {
    let s = fs::read_to_string("test.txt").unwrap();

    let mut score = 0;

    for line in s.lines(){
        let assign : Vec<&str> = line.split(",").collect();

        let range1 : Vec<&str> = assign[0].split("-").collect();
        let range2 : Vec<&str> = assign[1].split("-").collect();

        let lo1 = range1[0].parse::<i64>().unwrap();
        let hi1 = range1[1].parse::<i64>().unwrap();
        let lo2 = range2[0].parse::<i64>().unwrap();
        let hi2 = range2[1].parse::<i64>().unwrap();

        if is_contained(lo1, hi1, lo2, hi2){
            //println!("{}{}{}{}", lo1, hi1, lo2, hi2);
            score += 1;
        }
    }

    println!("{score}");
}
