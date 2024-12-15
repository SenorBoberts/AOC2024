use std::fs;
use std::collections::HashMap;

fn read_lists() -> (Vec<i32>, Vec<i32>){
    let f = fs::read_to_string("src/input.txt").expect("file not found");
    let v: Vec<&str> = f.split(char::is_whitespace).filter(|val| !val.is_empty()).collect();
    let mut l1 = vec!();
    let mut l2 = vec!();

    let mut i = 0;
    for x in v{
        if i % 2 == 0{
            l1.push(x.parse::<i32>().unwrap());
        }else{
            l2.push(x.parse::<i32>().unwrap());
        }
        i+=1;
    }
    return (l1, l2);
}

fn part1(l1: &mut Vec<i32>, l2: &mut Vec<i32>) -> i32{
    let mut dis = 0;
    l1.sort();
    l2.sort();
    for (v1, v2) in l1.iter().zip(l2.iter()){
        dis += (v2 - v1).abs();
    }
    return dis;
}

fn part2(l1: &mut Vec<i32>, l2: &mut Vec<i32>) -> i32{
    let mut map = HashMap::new();
    for v in l2{
        if map.contains_key(v){
            map.insert(*v, map.get(v).unwrap() + 1);
        }else{
            map.insert(*v, 1);
        }
    }

    let mut total = 0;
    for v in l1{
        if map.contains_key(v){
            total += *v * map.get(v).unwrap();
        }
    }

    return total;
}

fn main() {
    let (mut l1, mut l2) = read_lists();
    let dis = part1(&mut l1, &mut l2);
    println!("{}", dis);
    let total = part2(&mut l1, &mut l2);
    println!("{}", total);
}
