fn main() {
    let xs = vec![8, 23, 9, 0, 12, 11, 1, 10, 13, 7, 4, 14, 21, 5, 17, 3, 19, 2, 6];
    println!("{:?}", minfree(xs));
}


pub fn minfree(xs: Vec<usize>) -> usize {
    search(checklist(xs))
}

pub fn search(checklist: Vec<bool>) -> usize {
    checklist.iter().take_while(|&&v| v).count()
}

pub fn checklist(xs: Vec<usize>) -> Vec<bool> {
    let n = xs.iter().max().unwrap();
    let mut ret: Vec<bool> = vec![false; *n + 1];
    for x in xs.into_iter() {
        ret[x] = true;
    }

    return ret
}
