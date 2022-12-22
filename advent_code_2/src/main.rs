use std::fs;

#[derive(Debug)]
enum Move{
    Rock,
    Paper,
    Scissor
}

fn get_move(s : &str) -> Move{
    if s == "X" || s == "A"{
        Move::Rock
    }else if s == "B" || s == "Y"{
        Move::Paper
    }else{
        Move::Scissor
    }
}

fn play(m1 : Move, m2 : Move) -> u64{
    match m1{
        Move::Rock => match m2{
            Move::Rock => 1 + 3,
            Move::Paper => 1,
            Move::Scissor => 1 + 6
        },
        Move::Paper =>  match m2{
            Move::Rock => 2 + 6,
            Move::Paper => 2 + 3,
            Move::Scissor => 2
        },
        Move::Scissor => match m2{
            Move::Rock => 3,
            Move::Paper => 3 + 6,
            Move::Scissor => 3 + 3
        }
    }
}

fn eval_issue(m : Move, issue : &str) -> u64 {
    match m {
        Move::Rock =>{
            if issue == "X"{
                3
            }else if issue == "Y"{
                1 + 3
            }else{
                2 + 6
            }
        }
        Move::Paper => {
            if issue == "X"{
                1
            }else if issue == "Y"{
                2 + 3
            }else{
                3 + 6
            }
        },
        Move::Scissor =>{
            if issue == "X"{
                2
            }else if issue == "Y"{
                3 + 3
            }else{
                1 + 6
            }
        }
    }
}

fn main() {
    let s = fs::read_to_string("test.txt").unwrap();

    let mut score = 0;

    for line in s.lines(){
        let v : Vec<&str> = line.split(" ").collect();
        //let my_move = get_move(v[1]);
        let adv_move = get_move(v[0]);
        let issue = v[1];

        score += eval_issue(adv_move,issue);

        //score += play(my_move, adv_move);
    }

    println!("{score}");
}
