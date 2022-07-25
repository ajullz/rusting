#![cfg(test)]

use crate::problem1::{sum, dedup, filter};
use crate::problem2::{mat_mult, Matrix};
//use problem3::sieve;
//use problem4::{hanoi, Peg};

//
// Problem 1
//

// Part 1

#[test]
fn test_sum_zero() {
    let array = [1,2,3,4,5,-5,-4,-3,-2,-1,100,-100];
    assert_eq!(sum(&array), 0);
}

// Part 2

#[test]
fn test_dedup_once() {
    let vs = vec![1,1,1,1,1];
    assert_eq!(dedup(&vs), vec![1]);
}

// Part 3

fn negative_predicate(x: i32) -> bool {
    x < 0
}

#[test]
fn test_filter_negative() {
    let vs = vec![1,-2,3,-4,5];
    assert_eq!(filter(&vs, &negative_predicate), vec![-2,-4]);
}

//
// Problem 2
//

#[test]
#[should_panic]
fn test_mat_mult_empty() {
    let mat1 = Matrix::new();
    let mat2 = vec![vec![0f32; 3]; 4];
    mat_mult(&mat1, &mat2);
}

#[test]
#[should_panic]
fn test_mat_mult_not_matching_dims() {
    let mat1 = vec![vec![0f32; 3]; 4];
    let mat2 = vec![vec![0f32; 3]; 4];
    mat_mult(&mat1, &mat2);
}

#[test]
fn test_mat_mult_good_case() {
    let mat1 = vec![
        vec![1f32, 2f32, 3f32],
        vec![4f32, 5f32, 6f32],
    ];
    let mat2 = vec![
        vec![1f32, 4f32],
        vec![2f32, 5f32],
        vec![3f32, 6f32],
    ];
    let expected = vec![
        vec![14f32, 32f32],
        vec![32f32, 77f32]
    ];
    assert_eq!(mat_mult(&mat1, &mat2), expected);
}

/*

//
// Problem 3
//

#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2,3,5,7,11], sieve(12));
}

//
// Problem 4
//

#[test]
fn test_hanoi_1_disks() {
    let result = hanoi(1, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::C)], result);
    assert_eq!(1, result.len());
}

*/