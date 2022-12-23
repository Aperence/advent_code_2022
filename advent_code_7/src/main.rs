use std::{fs, collections::HashMap, path::PathBuf, u64::MAX};

#[derive(Debug)]
enum Files{
    File(u64),
    Dir(Vec<String>),
}

fn parse_str(iter : Vec<&str>) -> HashMap<String, Files>{
    let mut map = HashMap::new();
    let mut path = PathBuf::from("/");
    let mut index = 1;
    while index < iter.len(){
        let line = iter[index];
        if line == "$ cd .."{
            path.pop();
        }
        if line.starts_with("$ cd") && line != "$ cd .."{
            let words : Vec<&str> = line.split(" ").collect();
            path.push(words[2].to_owned() + "/");
        }
        if line == "$ ls"{
            let mut v = vec![];
            index += 1;
            while index < iter.len() && !iter[index].starts_with("$"){
                if iter[index].starts_with("dir"){
                    let mut dir = String::from(path.to_str().unwrap());
                    dir.push_str(iter[index].strip_prefix("dir ").unwrap());
                    dir.push_str("/");
                    v.push(dir);
                }else{
                    let words : Vec<&str> = iter[index].split(" ").collect();
                    let mut file = String::from(path.to_str().unwrap());
                    file.push_str(words[1]);
                    map.insert(file.clone(), Files::File(words[0].parse::<u64>().unwrap()));
                    v.push(file);
                }
                index += 1;
            }
            index -= 1;
            map.insert(String::from(path.to_str().unwrap()), Files::Dir(v));
        }
        index += 1;
    }

    map

}

fn size_dir_help(map : &HashMap<String, Files>, map2 : &mut HashMap<String, u64>, path : &String){
    let dir = map.get(path).unwrap();

    match dir{
        Files::File(_) => (),
        Files::Dir(v) =>{
            for str in v.iter(){
                size_dir_help(map, map2, str)
            }
            let mut sum = 0;
            for str in v.iter(){
                if map2.contains_key(str){
                    sum += map2.get(str).unwrap();
                }else{
                    if let Files::File(x) = map.get(str).unwrap(){
                        sum += *x;
                    }
                }
            }
            map2.insert(path.clone(), sum);
        }
    }
}

fn size_dir(map : &HashMap<String, Files>, path : String) -> HashMap<String, u64>{
    let mut ret = HashMap::new();
    size_dir_help(map, &mut ret, &path);
    ret
}


fn get_size_at_most(map : &HashMap<String, u64>, n : u64) -> u64{
    let mut sum = 0;
    for (_, size) in map.iter(){
        if *size < n{
            sum += *size;
        }
    }
    sum
}

fn get_delete_dir(map : &HashMap<String, u64>, needed_space : u64) -> u64{
    let mut min : u64 = std::u64::MAX;
    for (_, size) in map.iter(){
        if *size > needed_space && *size < min{
            min = *size;
        }
    }
    min
}

fn main() {
    let str = fs::read_to_string("test.txt").unwrap();

    let iter : Vec<&str> = str.lines().collect();

    let map = parse_str(iter);

    //println!("{:?}", map);

    let map_size = size_dir(&map, String::from("/"));

    let v = get_size_at_most(&map_size, 100000);

    let min = get_delete_dir(&map_size, 30000000 - (70000000 - map_size.get("/").unwrap()));

    println!("{:?} {}", v, min);
}
