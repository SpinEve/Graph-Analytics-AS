use csv::{self, Error};
use rand::{prelude::ThreadRng, Rng};
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::vec;

const EMPTY_VEC: Vec<usize> = Vec::new();
const N: usize = 2000000;

fn rand_walk(
    adj_list: &Vec<Vec<usize>>,
    n: usize,
    is_pub: &Vec<bool>,
    r: usize,
    init_v: usize,
    v_list: &mut Vec<usize>,
    pubd_list: &mut Vec<f64>,
    deg_list: &mut Vec<f64>,
    phi_list1: &mut Vec<f64>,
    phi_list2: &mut Vec<f64>,
) {
    let mut rng = rand::thread_rng();
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];

    let mut c = vec![0; n + 1];
    let mut d = vec![0; n + 1];
    // let mut e = vec![0; n + 1];

    let mut mark = vec![false; n + 1];
    let mut curr_v = init_v;
    for i in 0..r {
        v_list.push(curr_v);
        let mut flag = false;
        let mut next_v = 0;
        while !flag {
            // Randomly choose a neighbor
            let k = rng.gen_range(0..adj_list[curr_v].len());
            next_v = adj_list[curr_v][k];
            b[curr_v] += 1;
            if mark[next_v] {
                d[curr_v] += 1;
            }
            if is_pub[next_v] {
                // If it's public, choose it
                a[curr_v] += 1;
                flag = true;
                if mark[next_v] {
                    c[curr_v] += 1;
                }
            }
        }
        if i >= 1 {
            // for v in &adj_list[curr_v] {
            //     if mark[*v] {
            //         e[curr_v] += 1;
            //     }
            // }
            for v in &adj_list[v_list[i - 1]] {
                mark[*v] = false;
            }
        }
        for v in &adj_list[curr_v] {
            mark[*v] = true;
        }
        curr_v = next_v;
    }

    for i in 0..r {
        let v = v_list[i];
        let d = adj_list[v].len();
        let pd = (d as f64) * (a[v] as f64) / (b[v] as f64);
        pubd_list.push(pd);
        deg_list.push(d as f64);
    }
    for i in 1..r - 1 {
        let v = v_list[i];
        // let tmp1 = (c[v] as f64) / (a[v] as f64);
        // phi_list1.push(tmp1);
        let tmp2 = (d[v] as f64) / (b[v] as f64);
        phi_list2.push(tmp2);
        // phi_list.push((e[v] as f64) / (a[v] as f64));
    }
    for i in 1..r - 1 {
        let mut flag = false;
        for v in &adj_list[v_list[i - 1]] {
            if *v == v_list[i + 1] {
                flag = true;
                break;
            }
        }
        if flag {
            phi_list1.push(1.0);
        } else {
            phi_list1.push(0.0);
        }
    }
}

fn bfs(
    adj_list: &Vec<Vec<usize>>,
    is_pub: &Vec<bool>,
    mark: &mut Vec<usize>,
    init_v: usize,
) -> usize {
    if mark[init_v] != 0 || !is_pub[init_v] {
        return 0;
    }
    let mut deq = VecDeque::new();
    deq.push_back(init_v);
    mark[init_v] = init_v;
    let mut sum = 0;
    while !deq.is_empty() {
        let v = deq.pop_front().unwrap();
        sum += 1;
        for w in &adj_list[v] {
            if mark[*w] == 0 && is_pub[*w] {
                deq.push_back(*w);
                mark[*w] = init_v;
            }
        }
    }
    return sum;
}

fn find_lpc(adj_list: &Vec<Vec<usize>>, n: usize, is_pub: &Vec<bool>, lpc: &mut Vec<usize>) {
    let mut mark = vec![0; n + 1];
    let mut max_siz = 0;
    let mut max_v = 0;
    for v in 1..n + 1 {
        let siz = bfs(&adj_list, &is_pub, &mut mark, v);
        if siz > max_siz {
            max_siz = siz;
            max_v = v;
        }
    }
    for v in 1..n + 1 {
        if mark[v] == max_v {
            lpc.push(v);
        }
    }
}

fn exact_avgdeg(adj_list: &Vec<Vec<usize>>, n: usize) -> f64 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += adj_list[i].len();
    }
    return (sum as f64) / (n as f64);
}

fn exact_lpcavgdeg(adj_list: &Vec<Vec<usize>>, is_pub: &Vec<bool>, lpc: &Vec<usize>) -> f64 {
    let mut sum = 0;
    for v in lpc {
        for w in &adj_list[*v] {
            if is_pub[*w] {
                sum += 1;
            }
        }
    }
    return (sum as f64) / (lpc.len() as f64);
}

fn exact_globalcoeff(adj_list: &Vec<Vec<usize>>, n: usize) -> f64 {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 1..n + 1 {
        let d = adj_list[i].len();
        sum1 += d * (d - 1);
    }
    let mut marked = vec![false; n + 1];
    for u in 1..n + 1 {
        for v in &adj_list[u] {
            marked[*v] = true;
        }
        for v in &adj_list[u] {
            for w in &adj_list[*v] {
                if marked[*w] {
                    sum2 += 2;
                }
            }
            marked[*v] = false;
        }
    }
    return (sum2 as f64) / (sum1 as f64);
}

fn exact_avgcoeff(adj_list: &Vec<Vec<usize>>, n: usize) -> f64 {
    let mut sum = 0.0;
    let mut marked = vec![false; n + 1];
    for u in 1..n + 1 {
        let mut local_coeff = 0;
        let d = adj_list[u].len();
        if d <= 1 {
            continue;
        }
        for v in &adj_list[u] {
            marked[*v] = true;
        }
        for v in &adj_list[u] {
            for w in &adj_list[*v] {
                if marked[*w] {
                    local_coeff += 2;
                }
            }
            marked[*v] = false;
        }
        sum += (local_coeff as f64) / ((d * (d - 1)) as f64);
    }
    return sum / (n as f64);
}

fn exact_lpcavgcoeff(
    adj_list: &Vec<Vec<usize>>,
    n: usize,
    is_pub: &Vec<bool>,
    lpc: &Vec<usize>,
) -> f64 {
    let mut sum = 0.0;
    let mut marked = vec![false; n + 1];

    for u in lpc {
        // for u in 1..n + 1 {
        let mut local_coeff = 0;
        let mut pubd = 0;
        for v in &adj_list[*u] {
            if is_pub[*v] {
                pubd += 1;
            }
        }
        if pubd <= 1 {
            continue;
        }
        for v in &adj_list[*u] {
            if is_pub[*v] {
                marked[*v] = true;
            }
        }
        for v in &adj_list[*u] {
            if is_pub[*v] {
                for w in &adj_list[*v] {
                    if is_pub[*w] && marked[*w] {
                        local_coeff += 2;
                    }
                }
                marked[*v] = false;
            }
        }
        sum += (local_coeff as f64) / ((pubd * (pubd - 1)) as f64);
    }
    return sum / (lpc.len() as f64);
    // return sum / (n as f64);
}

fn smooth_avgd(deg_list: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for d in deg_list {
        sum += 1.0 / d;
    }
    (deg_list.len() as f64) / sum
}

fn rand_adj(adj_list: &mut Vec<Vec<usize>>, n: usize, p: f64, rng: &mut ThreadRng) {
    // Random Graph
    for i in 1..n + 1 {
        for j in i + 1..n + 1 {
            let rv: f64 = rng.gen();
            if rv <= p {
                adj_list[i].push(j);
                adj_list[j].push(i);
            }
        }
    }
}

fn init_pub(is_pub: &mut Vec<bool>, n: usize, rng: &mut ThreadRng, p_pri: f64) {
    let p = p_pri;
    for i in 1..n + 1 {
        let rv: f64 = rng.gen();
        if rv <= p {
            is_pub[i] = false;
        }
    }
}

fn test_avg_degree(
    adj_list: &Vec<Vec<usize>>,
    is_pub: &Vec<bool>,
    n: usize,
    lpc: &Vec<usize>,
    rng: &mut ThreadRng,
) -> (f64, f64) {
    let mut init_v;

    let t = 100;
    let r = n / 100;

    let mut pubd_list: Vec<f64> = Vec::with_capacity(r);
    let mut deg_list: Vec<f64> = Vec::with_capacity(r);
    let mut v_list: Vec<usize> = Vec::with_capacity(r);
    let mut phi_list1: Vec<f64> = Vec::with_capacity(r);
    let mut phi_list2: Vec<f64> = Vec::with_capacity(r);

    let mut s1 = 0.0;
    let mut s2 = 0.0;
    for _ in 0..t {
        init_v = lpc[rng.gen_range(0..lpc.len())];
        pubd_list.clear();
        deg_list.clear();
        v_list.clear();
        phi_list1.clear();
        phi_list2.clear();
        rand_walk(
            &adj_list,
            n,
            &is_pub,
            r,
            init_v,
            &mut v_list,
            &mut pubd_list,
            &mut deg_list,
            &mut phi_list1,
            &mut phi_list2,
        );
        // Smooth degree estimation
        let s_avgd = smooth_avgd(&pubd_list);
        // Proposed
        let pro_avgd = smooth_avgd(&deg_list);
        s1 += s_avgd;
        s2 += pro_avgd;
    }
    s1 /= t as f64;
    s2 /= t as f64;
    return (s1, s2);
}

fn nc_size(n: usize, v_list: &Vec<usize>, pubd_list: &Vec<f64>, m: usize) -> f64 {
    let mut sum1 = 0;
    let mut sum2 = 0.0;

    let r = v_list.len();

    let mut cntv = vec![0; n + 1];
    for i in m..r {
        cntv[v_list[i - m]] += 1;
        sum1 += cntv[v_list[i]] * 2;
    }

    // Prefix sum
    let mut prefix_sumd: Vec<f64> = Vec::with_capacity(r + 1);
    prefix_sumd.push(0.0);
    for i in 0..r {
        prefix_sumd.push(prefix_sumd[i] + (1.0 / pubd_list[i]));
    }

    for i in 0..r {
        // (i+m, r)
        sum2 += pubd_list[i] * (prefix_sumd[r] - prefix_sumd[cmp::min(r, i + m)]);
        // (0, i-m)
        if i + 1 >= m {
            sum2 += pubd_list[i] * prefix_sumd[i + 1 - m];
        }
    }
    return sum2 / (sum1 as f64);
}

fn pro_size(
    n: usize,
    v_list: &Vec<usize>,
    pubd_list: &Vec<f64>,
    deg_list: &Vec<f64>,
    m: usize,
) -> f64 {
    let mut sum1 = 0;
    let mut sum2 = 0.0;

    let r = v_list.len();

    let mut cntv = vec![0; n + 1];
    for i in m..r {
        cntv[v_list[i - m]] += 1;
        sum1 += cntv[v_list[i]] * 2;
    }

    let mut prefix_sumd: Vec<f64> = Vec::with_capacity(r + 1);
    prefix_sumd.push(0.0);
    for i in 0..r {
        prefix_sumd.push(prefix_sumd[i] + (1.0 / pubd_list[i]));
    }

    for i in 0..r {
        sum2 += deg_list[i] * (prefix_sumd[r] - prefix_sumd[cmp::min(r, i + m)]);
        if i + 1 >= m {
            // Using real degree rather than public degree
            sum2 += deg_list[i] * prefix_sumd[i + 1 - m];
        }
    }

    // Naive implementation O(r^2), for correctness comparison
    // let mut sum3 = 0.0;
    // let mut sum4 = 0;
    // for i in 0..v_list.len() {
    //     for j in i + m..v_list.len() {
    //         sum3 += deg_list[i] / pubd_list[j];
    //         sum3 += deg_list[j] / pubd_list[i];
    //         if v_list[i] == v_list[j] {
    //             sum4 += 2;
    //         }
    //     }
    // }
    // println!("{}={}, {}={}?", sum1, sum4, sum2, sum3);

    return sum2 / (sum1 as f64);
}

fn test_size(
    adj_list: &Vec<Vec<usize>>,
    is_pub: &Vec<bool>,
    n: usize,
    lpc: &Vec<usize>,
    rng: &mut ThreadRng,
) -> (f64, f64) {
    let mut init_v;

    let t = 100; //1000

    let r;
    let m;
    if n < 200000 {
        r = 2000;
        m = 100;
    } else {
        r = n / 100;
        m = ((r as f64) * 0.025) as usize;
    }

    let mut pubd_list: Vec<f64> = Vec::with_capacity(r);
    let mut deg_list: Vec<f64> = Vec::with_capacity(r);
    let mut v_list: Vec<usize> = Vec::with_capacity(r);
    let mut phi_list1: Vec<f64> = Vec::with_capacity(r);
    let mut phi_list2: Vec<f64> = Vec::with_capacity(r);

    let mut s1 = 0.0;
    let mut s2 = 0.0;

    println!("m = {}", m);

    for _ in 0..t {
        init_v = lpc[rng.gen_range(0..lpc.len())];
        pubd_list.clear();
        deg_list.clear();
        v_list.clear();
        phi_list1.clear();
        phi_list2.clear();
        rand_walk(
            &adj_list,
            n,
            &is_pub,
            r,
            init_v,
            &mut v_list,
            &mut pubd_list,
            &mut deg_list,
            &mut phi_list1,
            &mut phi_list2,
        );
        // NC algorithm
        let nc_size = nc_size(n, &v_list, &pubd_list, m);
        s1 += nc_size;
        // Proposed
        let pro_size = pro_size(n, &v_list, &pubd_list, &deg_list, m);
        s2 += pro_size;
    }
    s1 /= t as f64;
    s2 /= t as f64;
    return (s1, s2);
}

fn global_coeff(deg_list: &Vec<f64>, phi_list: &Vec<f64>) -> f64 {
    let mut sum1 = 0.0;
    let r = deg_list.len();
    for deg in deg_list {
        sum1 += *deg - 1.0;
    }
    sum1 /= r as f64;

    let mut sum2 = 0.0;
    for i in 1..r - 1 {
        sum2 += deg_list[i] * (phi_list[i - 1] as f64);
    }
    sum2 /= (r - 2) as f64;
    return sum2 / sum1;
}

fn avg_coeff(deg_list: &Vec<f64>, phi_list: &Vec<f64>) -> f64 {
    let mut sum1 = 0.0;
    let r = deg_list.len();
    for deg in deg_list {
        sum1 += 1.0 / *deg;
    }
    sum1 /= r as f64;

    let mut sum2 = 0.0;
    for i in 1..r - 1 {
        if deg_list[i] > 1.0 {
            // sum2 += (phi_list[i - 1] as f64) / (deg_list[i] * (deg_list[i] - 1.0));
            sum2 += (phi_list[i - 1] as f64) / (deg_list[i] - 1.0);
        }
    }
    sum2 /= (r - 2) as f64;
    return sum2 / sum1;
}

fn test_avg_coeff(
    adj_list: &Vec<Vec<usize>>,
    is_pub: &Vec<bool>,
    n: usize,
    lpc: &Vec<usize>,
    rng: &mut ThreadRng,
) -> (f64, f64) {
    let mut init_v;

    let t = 100; // Default val: 1000
    let r = n / 100;

    let mut pubd_list: Vec<f64> = Vec::with_capacity(r);
    let mut deg_list: Vec<f64> = Vec::with_capacity(r);
    let mut v_list: Vec<usize> = Vec::with_capacity(r);
    let mut phi_list1: Vec<f64> = Vec::with_capacity(r);
    let mut phi_list2: Vec<f64> = Vec::with_capacity(r);

    let mut s1 = 0.0;
    let mut s2 = 0.0;

    for _ in 0..t {
        init_v = lpc[rng.gen_range(0..lpc.len())];
        pubd_list.clear();
        deg_list.clear();
        v_list.clear();
        phi_list1.clear();
        phi_list2.clear();
        rand_walk(
            &adj_list,
            n,
            &is_pub,
            r,
            init_v,
            &mut v_list,
            &mut pubd_list,
            &mut deg_list,
            &mut phi_list1,
            &mut phi_list2,
        );
        // Original
        let coeff1 = avg_coeff(&pubd_list, &phi_list1);
        s1 += coeff1;
        // Proposed
        let coeff2 = avg_coeff(&deg_list, &phi_list2);
        s2 += coeff2;
    }
    s1 /= t as f64;
    s2 /= t as f64;
    return (s1, s2);
}

fn test_global_coeff(
    adj_list: &Vec<Vec<usize>>,
    is_pub: &Vec<bool>,
    n: usize,
    lpc: &Vec<usize>,
    rng: &mut ThreadRng,
    p_pri: f64,
) {
    let mut init_v;

    let t = 100; // Default val: 1000
    let r = n / 100;

    let mut pubd_list: Vec<f64> = Vec::with_capacity(r);
    let mut deg_list: Vec<f64> = Vec::with_capacity(r);
    let mut v_list: Vec<usize> = Vec::with_capacity(r);
    let mut phi_list1: Vec<f64> = Vec::with_capacity(r);
    let mut phi_list2: Vec<f64> = Vec::with_capacity(r);

    let mut s1 = 0.0;

    for _ in 0..t {
        init_v = lpc[rng.gen_range(0..lpc.len())];
        pubd_list.clear();
        deg_list.clear();
        v_list.clear();
        phi_list1.clear();
        phi_list2.clear();
        rand_walk(
            &adj_list,
            n,
            &is_pub,
            r,
            init_v,
            &mut v_list,
            &mut pubd_list,
            &mut deg_list,
            &mut phi_list1,
            &mut phi_list2,
        );
        //
        let coeff = global_coeff(&pubd_list, &phi_list1);
        s1 += coeff;
    }
    s1 /= t as f64;
    println!("Original algorithm: {}", s1);
}

fn read_data(
    adj_list: &mut Vec<Vec<usize>>,
    n: &mut usize,
    name: &str,
    bi_edge: bool,
) -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_path("./data/".to_owned() + name)?;
    let mut num = 0;
    let mut idx = HashMap::new();
    for res in rdr.records() {
        let record = res?;
        let u: usize = record[0].parse().unwrap();
        let v: usize = record[1].parse().unwrap();
        if !idx.contains_key(&u) {
            num += 1;
            idx.insert(u, num);
        }
        if !idx.contains_key(&v) {
            num += 1;
            idx.insert(v, num);
        }
        adj_list[*idx.get(&u).unwrap()].push(*idx.get(&v).unwrap());
        if !bi_edge {
            adj_list[*idx.get(&v).unwrap()].push(*idx.get(&u).unwrap());
        }
    }
    *n = num;
    Ok(())
}

fn main() {
    // let mut o_file = File::create("output.txt").unwrap();
    let mut rng = rand::thread_rng();
    let mut adj_list = vec![EMPTY_VEC; N];

    // let n = 10000;
    // rand_adj(&mut adj_list, n, 50.0 / (n as f64 - 1.0), &mut rng);

    let mut n = 0;
    let file_name = "GitHub.csv"; // "CA-GrQc.csv", "p2p-Gnutella04.csv", p2p-Gnutella31.csv
    read_data(&mut adj_list, &mut n, file_name, false).ok(); // If data has bidirectional edge, set bi_edge to true
    println!("Graph size n: {}", n);

    let mut lpc: Vec<usize> = Vec::with_capacity(n);

    let mut p_pri = 0.0;

    let ex_avgd = exact_avgdeg(&adj_list, n);
    println!("Exact average degree: {}", ex_avgd);

    let ex_gcoeff = exact_globalcoeff(&adj_list, n);
    println!("Exact global coeff: {}", ex_gcoeff);

    let ex_avgcoeff = exact_avgcoeff(&adj_list, n);
    println!("Exact average coeff: {}", ex_avgcoeff);

    println!("p,avgd,avgd*,orid,prod,avgc,avgc*,oriavgc,proavgc");
    while p_pri <= 0.8 {
        let mut is_pub = vec![true; n + 1];
        init_pub(&mut is_pub, n, &mut rng, p_pri);
        // println!("{:?}", adj_list);

        lpc.clear();
        find_lpc(&adj_list, n, &is_pub, &mut lpc);
        // println!("{:?}", lpc);

        let ex_lpcavgd = exact_lpcavgdeg(&adj_list, &is_pub, &lpc);
        let (ori_d, pro_d) = test_avg_degree(&adj_list, &is_pub, n, &lpc, &mut rng);

        let ex_lpcsize = lpc.len();
        // test_size(&adj_list, &is_pub, n, &lpc, &mut rng);

        let ex_lpcavgcoeff = exact_lpcavgcoeff(&adj_list, n, &is_pub, &lpc);

        let (ori_avgc, pro_avgc) = test_avg_coeff(&adj_list, &is_pub, n, &lpc, &mut rng);
        // test_global_coeff(&adj_list, &is_pub, n, &lpc, &mut rng, p_pri);

        println!(
            "{},{},{},{},{},{},{},{},{}",
            p_pri,
            ex_avgd,
            ex_lpcavgd,
            ori_d,
            pro_d,
            ex_avgcoeff,
            ex_lpcavgcoeff,
            ori_avgc,
            pro_avgc
        );

        p_pri += 0.1;
    }
}
