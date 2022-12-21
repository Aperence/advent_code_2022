use std::{fs, collections::HashMap};

#[derive(Debug, Clone)]
enum Op{
    Term(i64),
    BinOp(String, String, String),
    Unknown
}


fn evaluate<'a>(name : &String, map : &HashMap<String, Op>) -> i64 {
    let op = map.get(name).unwrap();
    match op {
        Op::Term(x) => *x,
        Op::BinOp(operand, first_name, second_name) => {
            let one = evaluate(first_name, map);
            let two = evaluate(second_name, map);

            apply_op(one, two, operand)
        },
        Op::Unknown => 0
    }
}

fn apply_op(i : i64, j : i64, operand : &String) -> i64{
    if operand == "+"{
        i+j
    }else if operand == "-"{
        i-j
    }else if operand == "*"{
        i*j
    }else{
        i/j
    }
}

// left <==> left term is unknown
fn inv_apply_op(known : i64, res : i64, operand : &String, left : bool) -> i64{
    if operand == "+"{
        res - known
    }else if operand == "-" && left{
        known+res
    }else if operand == "-" && !left{
        known - res
    }else if operand == "*"{
        res / known
    }else if operand == "/" && left{
        res*known
    }else{
        known / res
    }
}

fn evaluateTop<'a>(name : &String, mut map : HashMap<String, Op>) -> HashMap<String, Op>{
    if name == "humn"{
        *map.entry(name.clone()).or_insert(Op::Term(0)) = Op::Unknown;
        map
    }else{
        let op = map.get(name).unwrap().clone();
        match op {
            Op::Term(x) => map,
            Op::BinOp(operand, first_name, second_name) => {
                let mut new_map1 = evaluateTop(&first_name, map);
                let mut new_map2 = evaluateTop(&second_name, new_map1);

                if let Op::Term(y) = new_map2.get(&first_name).unwrap(){
                    if let Op::Term(z) = new_map2.get(&second_name).unwrap(){
                        if *y != -10000 &&  *z != -10000{
                            *new_map2.entry(name.clone()).or_insert(Op::Term(0)) = Op::Term(apply_op(*y, *z, &operand));
                        }
                    }
                }
                new_map2
            },
            Op::Unknown => map
        }
    }
}

fn evaluateDown<'a>(name : &String, map : &HashMap<String, Op>, acc : i64) -> i64 {
    if name == "humn"{
        return acc;
    }
    let op = map.get(name).unwrap();
    match op {
        Op::Term(x) => *x,
        Op::BinOp(operand, first_name, second_name) => {
            match map.get(first_name).unwrap() {
                Op::Term(x) => {
                    evaluateDown(second_name, map, inv_apply_op(*x, acc, operand, false))
                },
                Op::BinOp(_, _, _) => {
                    if let Op::Term(y) = map.get(second_name).unwrap(){
                        evaluateDown(first_name, map, inv_apply_op(*y, acc, operand, true))
                    }else{
                        -1
                    }
                }
                Op::Unknown => {
                    if let Op::Term(y) = map.get(second_name).unwrap(){
                        evaluateDown(first_name, map, inv_apply_op(*y, acc, operand, true))
                    }else{
                        -1
                    }
                }
            }
        }
        Op::Unknown => 0
    }
}

fn main() {
    let str = fs::read_to_string("test.txt").unwrap();

    let mut map : HashMap<String, Op> = HashMap::new();
    for line in str.lines(){
        let v : Vec<&str>= line.split(":").collect();

        let name = String::from(v[0]);

        let temp = v[1].trim();

        let prev = temp.clone();

        let number = temp.parse::<i64>();

        let op = match number{
            Ok(num) => Op::Term(num),
            Err(_) => {
                let t : Vec<&str> = prev.split(" ").collect();
                Op::BinOp(String::from(t[1]), String::from(t[0]), String::from(t[2]))
            }
        };

        map.insert(name, op);
    }

    let new_map = evaluateTop(&String::from("root"), map);

    let mut name = "";
    let mut other_name = "";

    if let Op::BinOp(_, first, second) = new_map.get("root").unwrap(){
        if let Op::Term(_) = new_map.get(first).unwrap(){
            name = first;
            other_name = second;
        }else{
            name = second;
            other_name = first;
        }
    }

    let value = new_map.get(name).unwrap();

    let mut acc = 0i64;

    if let Op::Term(x) = value{
        acc = *x;
    }

    let res = evaluateDown(&String::from(other_name), &new_map, acc);
    println!("{:?}", res);

    
}
