use std::fs;
use priority_queue::PriorityQueue;

fn main() {
    let s = fs::read_to_string("test.txt").unwrap();

    let mut pq : PriorityQueue<i64, i64> = PriorityQueue::new();
    let mut current_sum = 0i64;
    for line in s.lines() {
        if line.trim() == ""{
            pq.push(-current_sum, -current_sum);
            if pq.len() == 4{
                pq.pop();
            }
            current_sum = 0;
        }else{
            current_sum += line.parse::<i64>().unwrap();
        }
    }

    println!("{:?}", pq);

    println!("{}", -pq.pop().unwrap().0 + -pq.pop().unwrap().0 + -pq.pop().unwrap().0);
}
