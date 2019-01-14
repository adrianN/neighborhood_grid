extern crate permutohedron;
extern crate rand;
use permutohedron::Heap;
use rand::seq::SliceRandom;
use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const NUMBER_OF_ELEMENTS: usize = 4;

fn fits(v: &[usize], pos: usize, matrix: &mut [usize], spot: usize) -> bool {
    let column_ok = if spot < NUMBER_OF_ELEMENTS {
        true
    } else {
        let m_pos = matrix[spot - NUMBER_OF_ELEMENTS];
        let l_value =
            v[pos]
        ;
        let r_value = 
            v[m_pos]
        ;
        l_value > r_value
    };
    if column_ok {
        let column = (spot) % NUMBER_OF_ELEMENTS;
        let row_ok = if column == 0 {
            true
        } else {
            matrix[spot - 1] < pos
        };
        row_ok
    } else {
        false
    }
}

fn fill_rec(
    v: &[usize],
    chosen: &mut [bool],
    matrix: &mut [usize],
    cutoff: u64,
    spot: usize,
) -> u64 {
    let mut count: u64 = 0;
    for i in 0..chosen.len() {
        if chosen[i] {
            continue;
        }
        if fits(v, i, matrix, spot) {
            if spot+1 >= NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS {
                count = count + 1;
            } else {
                chosen[i] = true;
                matrix[spot] = i;
                count = count.saturating_add(fill_rec(v, chosen, matrix, cutoff, spot + 1));
                chosen[i] = false;
            }
            if count > cutoff {
                return u64::max_value();
            }
        }
    }
    count
}
fn main() {
    let mut v: [usize; NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS] =
        [0; NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS];
    for i in 0..NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS {
        v[i] = i;
    }
    println!("{:?}", v);
    let mut matrix: [usize; NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS] =
        [0; NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS];
    let mut m = u64::max_value();
    {
        let mut chosen = [false; NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS];
        let mut rng = rand::thread_rng();
        let mut now = Instant::now();
        for i in 0..100000 {
            v.shuffle(&mut rng);
            let new_m = fill_rec(&v, &mut chosen, &mut matrix, m, 0);
            for i in 0..chosen.len() {
                chosen[i] = false;
            }
            if new_m <= m {
                m = new_m;
                println!("Permutation {:?}, count {}", v, m);
            }
            let new_now = Instant::now();
            if new_now.duration_since(now) > Duration::new(30,0) {
                println!("Preprocess {}", i);
                now = new_now
            }
        }
    }
    println!("Random sampling phase done");
    let cutoff = Arc::new(Mutex::new(m));

    let mut threads = Vec::new();
    for i in 0..v.len() {
        v.rotate_left(1);
        let cutoff = cutoff.clone();
        let mut u = v.clone();
        threads.push(thread::spawn(move || {
            let mut m = u64::max_value();
            let prefix = [u[0]];
            let mut chosen = [false; NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS];
            let heap = Heap::new(&mut u[1..]);

            let mut count = 0;
            let mut prev_count = 0;
            let mut now = Instant::now();
            let mut x = [0; NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS];
            x[0] = prefix[0];
            for p in heap {
                count += 1;
                if count % 1000 == 0 {
                    let new_now = Instant::now();
                    if new_now.duration_since(now) > Duration::new(5*60,0) {
                        {
                            m = min(m, *cutoff.lock().unwrap());
                        }
                        println!("Thread {} at {}, {}/s", i,count, (count-prev_count) as f32 / new_now.duration_since(now).as_secs() as f32 );
                        now = new_now;
                        prev_count = count;
                    }
                }
                x[1..].clone_from_slice(&p);
                let new_m = fill_rec(&x, &mut chosen, &mut matrix, m, 0);
                for i in 0..chosen.len() {
                    chosen[i] = false;
                }
                if new_m <= m {
                    m = new_m;
                    println!("Permutation {:?}, count {}", x, m);
                    {
                        let mut c = cutoff.lock().unwrap();
                        *c = min(*c, m);
                        m = *c;
                    }
                }
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}
