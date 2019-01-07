extern crate permutohedron;
extern crate rand;
use permutohedron::Heap;
use permutohedron::factorial;
use rand::seq::SliceRandom;

const NUMBER_OF_ELEMENTS: usize = 4;

fn fits(v: &[usize], pos: usize, matrix: &mut [usize], spot : usize) -> bool {
    if spot == 0 {
        return true;
    }
    let column = (spot) % NUMBER_OF_ELEMENTS;
    let row_ok = if column == 0 {
        true
    } else {
        matrix[spot - 1] < pos
    };
    let column_ok = if spot < NUMBER_OF_ELEMENTS {
        true
    } else {
        v[pos] > v[matrix[spot - NUMBER_OF_ELEMENTS]]
    };
    row_ok && column_ok
}


fn fill_rec(v: &[usize], chosen: &mut [bool], matrix: &mut [usize], cutoff : u64, spot : usize) -> u64 {
    let mut count: u64 = 0;
    let mut done = 0;
    for i in 0..v.len() {
        if chosen[i] {
            done += 1;
            continue;
        }
        if fits(v, i, matrix, spot) {
            chosen[i] = true;
            matrix[spot] = i;
            count = count.saturating_add(fill_rec(v, chosen, matrix, cutoff, spot+1));
            if count > cutoff {
                return u64::max_value();
            }
            chosen[i] = false;
        }
    }
    if done == chosen.len() {
       1
    } else {
        count
    }
}
fn main() {
    let mut v: [usize; NUMBER_OF_ELEMENTS*NUMBER_OF_ELEMENTS] = [0; NUMBER_OF_ELEMENTS*NUMBER_OF_ELEMENTS];
    for i in 0..NUMBER_OF_ELEMENTS * NUMBER_OF_ELEMENTS {
        v[i] = i;
    }
    println!("{:?}", v);
    let mut chosen : [bool; NUMBER_OF_ELEMENTS*NUMBER_OF_ELEMENTS] = [false; NUMBER_OF_ELEMENTS*NUMBER_OF_ELEMENTS];
    let mut matrix: [usize; NUMBER_OF_ELEMENTS*NUMBER_OF_ELEMENTS] = [0; NUMBER_OF_ELEMENTS*NUMBER_OF_ELEMENTS];
    let mut m = u64::max_value();
    let mut rng = rand::thread_rng();
    for _i in 0..10000 {
        v.shuffle(&mut rng);
        let new_m = fill_rec(&v, &mut chosen, &mut matrix, m, 0);
        for i in 0..chosen.len() {
            chosen[i] = false;
        }
        if new_m <= m {
            m = new_m;
            println!("Permutation {:?}, count {}", v, m);
        }
    }

    let heap = Heap::new(&mut v);
    let num_permutations = factorial(NUMBER_OF_ELEMENTS*NUMBER_OF_ELEMENTS)/100;
    let mut count = 0;
    let mut percent = 0;
    for p in heap {
        count += 1;
        if count > num_permutations {
            percent += 1;
            println!("{}% done!", percent);
            count = 0;
        }
        let new_m = fill_rec(&p, &mut chosen, &mut matrix, m, 0);
        for i in 0..chosen.len() {
            chosen[i] = false;
        }
        if new_m <= m {
            m = new_m;
            println!("Permutation {:?}, count {}", p, m);
        }
    }
}
