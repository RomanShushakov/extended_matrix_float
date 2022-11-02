#![allow(unused_imports)]

use crate::MyFloatTrait;


#[test]
fn test_my_powi()
{
    let v_1 = 2f32;
    let v_2 = -3f64;
    assert_eq!(v_1.my_powi(2), 4f32);
    assert_eq!(v_2.my_powi(3), -27f64);
}


#[test]
fn test_my_sqrt()
{
    let v_1 = 2f32;
    let v_2 = 9f64;
    assert_eq!(v_1.my_sqrt(), f32::sqrt(2.0));
    assert_eq!(v_2.my_sqrt(), 3f64);
}


#[test]
fn test_my_acos()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    assert_eq!(v_1.my_acos(), f32::acos(0.5));
    assert_eq!(v_2.my_acos(), f64::acos(-0.98));
}


#[test]
fn test_my_cos()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    assert_eq!(v_1.my_cos(), f32::cos(0.5));
    assert_eq!(v_2.my_cos(), f64::cos(-0.98));
}


#[test]
fn test_my_sin()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    assert_eq!(v_1.my_sin(), f32::sin(0.5));
    assert_eq!(v_2.my_sin(), f64::sin(-0.98));
}


#[test]
fn test_my_abs()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    assert_eq!(v_1.my_abs(), f32::abs(0.5));
    assert_eq!(v_2.my_abs(), f64::abs(-0.98));
}


#[test]
fn test_my_asin()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    assert_eq!(v_1.my_asin(), f32::asin(0.5));
    assert_eq!(v_2.my_asin(), f64::asin(-0.98));
}


#[test]
fn test_my_atan2()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f32;
    let v_3 = 0.5f64;
    let v_4 = -0.98f64;
    assert_eq!(v_1.my_atan2(&v_2), 0.5f32.atan2(-0.98f32));
    assert_eq!(v_3.my_atan2(&v_4), 0.5f64.atan2(-0.98f64));
}


#[test]
fn test_my_atan()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    assert_eq!(v_1.my_atan(), f32::atan(0.5));
    assert_eq!(v_2.my_atan(), f64::atan(-0.98));
}


#[test]
fn test_my_to_degrees()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    assert_eq!(v_1.my_to_degrees(), f32::to_degrees(0.5));
    assert_eq!(v_2.my_to_degrees(), f64::to_degrees(-0.98));
}


#[test]
fn test_my_is_nan()
{
    let v_1 = 0.5f32;
    let v_2 = -0.98f64;
    let v_3 = 1.5f32;
    let v_4 = -2.98f64;
    assert_eq!(v_1.my_acos().my_is_nan(), false);
    assert_eq!(v_2.my_acos().my_is_nan(), false);
    assert_eq!(v_3.my_acos().my_is_nan(), true);
    assert_eq!(v_4.my_acos().my_is_nan(), true);
}
