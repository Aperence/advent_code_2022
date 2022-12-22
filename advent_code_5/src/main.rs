use std::fs;

#[macro_use] extern crate text_io;

fn main() {
    let s = fs::read_to_string("test.txt").unwrap();

    let n_elem = 9;

    let mut stacks : Vec<Vec<char>> = vec![];

    for _ in 0..n_elem{
        stacks.push(vec![])
    }

    let mut starting = true;

    for line in s.lines(){
        if line.trim() == ""{
            starting = false;
            continue;
        }
        if starting {
            let mut offset = 1;
            for temp in 0..n_elem{
                if line.chars().nth(offset).unwrap() == '1'{
                    continue;
                }
                if line.chars().nth(offset).unwrap() != ' '{
                    stacks.get_mut(temp).unwrap().insert(0, line.chars().nth(offset).unwrap());
                }
                offset += 4;
            }
        }else{
            let (num, st1, st2) : (u64, u64, u64);
            scan!(line.bytes() => "move {} from {} to {}\n", num, st1, st2);

            /* 
            for _ in 0..num{
                let elem = stacks[(st1-1) as usize].pop().unwrap();
                stacks[(st2-1) as usize].push(elem);
            }*/

            let mut v = vec![];
            for _ in 0..num{
                let elem = stacks[(st1-1) as usize].pop().unwrap();
                v.push(elem);
            }

            for i in 0..num{
                stacks[(st2-1) as usize].push(v[(num - i - 1) as usize]);
            }
        }
    }

    let mut ret = String::new();

    for i in 0..n_elem{
        ret.push(stacks[i as usize].pop().unwrap());
    }

    println!("{ret}");
}
