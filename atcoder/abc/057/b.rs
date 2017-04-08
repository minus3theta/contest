#[allow(unused_imports)]
use std::io;
use std::io::stdin;

fn read_line_as_vec() -> Vec<String> {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("");
    s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let nm = read_line_as_vec();
    let n: i32 = nm[0].trim().parse().expect("n");
    let m: i32 = nm[1].trim().parse().expect("m");
    let mut students = Vec::<(i32,i32)>::new();
    for _ in 0..n {
        let ab = read_line_as_vec();
        let a: i32 = ab[0].trim().parse().expect("");
        let b: i32 = ab[1].trim().parse().expect("");
        students.push((a, b));
    }
    let students = students;
    let mut points = Vec::<(i32,i32)>::new();
    for _ in 0..m {
        let cd = read_line_as_vec();
        let c: i32 = cd[0].trim().parse().expect("");
        let d: i32 = cd[1].trim().parse().expect("");
        points.push((c, d));
    }
    let points = points;
    for (a,b) in students {
        let mut argmin_cp = 0;
        let mut min_dist: i32 = 1_000_000_000;
        for (i,&(c,d)) in points.iter().enumerate() {
            let dist: i32 = (c-a).abs() + (d-b).abs();
            if dist < min_dist {
                min_dist = dist;
                argmin_cp = i + 1;
            }
        }
        println!("{}", argmin_cp);
    }
}
