use std::time::SystemTime;
use std::thread;
use std::sync::{mpsc, Arc};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Matrix 1
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to readline");
    let n: usize = input.trim().parse().unwrap();
    let mut mat1: Vec<Vec<usize>> = vec![];
    for i in 0..n{
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to readline");
        let ia:Vec<&str> = input.trim().split(" ").collect();
        mat1.push(vec![]);
        for j in 0..ia.len(){
            mat1[i].push(ia[j].parse().unwrap());
        }
    }

    // Matrix 2
    input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to readline");
    let m: usize = input.trim().parse().unwrap();
    let mut mat2: Vec<Vec<usize>> = vec![];
    for i in 0..m{
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to readline");
        let ia:Vec<&str> = input.trim().split(" ").collect();
        mat2.push(vec![]);
        for j in 0..ia.len(){
            mat2[i].push(ia[j].parse().unwrap());
        }
    }

    // Time start
    let now = SystemTime::now();

    if args.len()>1{
        let _mat3 = mat_mul(&mat1, &mat2, n, m);
    }
    else{
        let _mat3 = mat_mul_op(&mat1, &mat2, n, m);
    }

    // Time end
    match now.elapsed() {
       Ok(elapsed) => {
           println!("Executed in {}s", elapsed.as_secs());
       }
       Err(e) => {
           println!("Error: {:?}", e);
       }
    }

    // print!("{:?}\n", mat3);
}

// Single Threaded
fn mat_mul(mat1: &Vec<Vec<usize>>, mat2: &Vec<Vec<usize>>, n: usize, m:usize) -> Vec<Vec<usize>>{
    #[allow(unused_mut)]
    let mut ans:Vec<Vec<usize>> = vec![];
    if n!=m{
        return ans;
    }
    let mut ans:Vec<Vec<usize>> = vec![];
    for i in 0..n{
        ans.push(vec![]);
        for j in 0..n{
            let mut cur = 0;
            for k in 0..n{
                cur += mat1[i][k]*mat2[k][j];
            }
            ans[i].push(cur);
        }
    }
    return ans; 
}

// Multi Threaded
fn mat_mul_op(mat1: &Vec<Vec<usize>>, mat2: &Vec<Vec<usize>>, n: usize, m:usize) -> Vec<Vec<usize>>{
    #[allow(unused_mut)]
    if n!=m{
        return vec![];
    }

    // Threading stuff
    let thread_count = 4;
    let mut threads: Vec<_> = Vec::new();
    let (tx, rx) = mpsc::channel();
    let amat1 = Arc::new(mat1.clone());
    let amat2 = Arc::new(mat2.clone());

    for th in 0..thread_count{
        let tx = tx.clone();
        let amat1 = Arc::clone(&amat1);
        let amat2 = Arc::clone(&amat2);
        threads.push( thread::spawn(move || {
            println!("thread-{} started!", th);

            let start = (th*n)/thread_count;
            let end = (n*(th+1))/thread_count;
            let mut ans:Vec<Vec<usize>> = vec![vec![]; n];
            for i in start..end{
                for j in 0..n{
                    let mut cur = 0;
                    for k in 0..n{
                        cur += amat1[i][k]*amat2[k][j];
                    }
                    ans[i].push(cur);
                }
            }
            tx.send((start, end, ans)).unwrap();
        }));
    }

    let mut ans:Vec<Vec<usize>> = vec![vec![]; n];
    for v in rx.iter().take(thread_count){
        let (start, end, mat) = v;
        for i in start..end{
            ans[i].extend(&mat[i]);
        }
    }

    for handle in threads {
        handle.join().unwrap();
    }

    return ans; 
}


