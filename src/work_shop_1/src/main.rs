use std::io;
use regex::Regex;
const SENTENCES: &str = "This is a regular paragraph with the default style of Normal. \
    This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default \
    style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph \
    with the default style of Normal.";
fn main() {
    let check = example_1();
    println!("check with parent: {:?} and child: {:?}, result: {:?}",[1, 2,3,5,6,8, 10, 11], [6,8,10], check);

    let check = example_2();
    println!("SENTEN to check: {:?}", SENTENCES);
    println!("resutl = {:?}", check);
}

// exercise 1
fn example_1 () -> bool {
    let parent = [1, 2,3,5,6,8, 10, 11];
    let child = [6,8,10];
    let mut check = false;
    if parent.len() > child.len() {
        for i in 0..parent.len() {
            if parent[i] == child[0] && i + child.len() - 1 < parent.len(){
                check = true;
                for j in 0..child.len() {
                    if parent[i + j] != child[j] {
                        check = false
                    }
                }
            }
        }
    } else {
        return false
    }
    if check {
        return true
    } else {
        return false
    }
}
// exercise 2
fn example_2 () -> i32 {
    let mut buf = String::new();
    println!("input your word here: ");
    io::stdin().read_line(&mut buf);

    let string_check = String::from(SENTENCES);
    let mut inter = string_check.split(buf.split('\n').next().unwrap());
    let mut len = 0;
    let mut check = inter.next();
    while check != None {
        check = inter.next();
        len += 1;
    }

    let rs = string_check;
    return (len - 1) as i32;
}
// exercise 2 plus
