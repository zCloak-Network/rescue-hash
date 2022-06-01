#![no_std]
use crypto::{hash::rescue::rp64_256::ElementDigest, Digest, ElementHasher};
// use log::debug;
use sp_std::{vec, vec::Vec};
// use serde::{Serialize, Deserialize};
#[macro_use]
extern crate alloc;
use alloc::string::String;
// RE-EXPORTS
// ================================================================================================

use math::fields::f64::BaseElement;
use math::StarkField;

extern crate console_error_panic_hook;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// extern crate web_sys;
use wasm_bindgen_test::*;

use hex;
// RESCUE
// ================================================================================================

///  Executes the `rescue` fucntion and returns the rescue hash result.
///
/// * `inputs` specifies the rescue input, it should contain 8 elements or more(over 8 but should be some multiple of 4);
/// *  Return the hash result Vec<u64>
#[wasm_bindgen]
pub fn rescue(values: String) -> Vec<u64> {
    let mut values_in_u64 = vec![];
    if values.len() != 0 {
        let values_a: Vec<&str> = values.split(',').collect();
        values_in_u64 = values_a
            .iter()
            // .map(|public_a| public_a.parse::<u64>().unwrap())
            .map(|public_a| public_a.parse::<u64>().expect("parse fail"))
            .collect();
    };
    assert!(
        (values_in_u64.len()== 8) || ( (values_in_u64.len() > 8) && (values_in_u64.len() % 4 == 0)),
        "expected len of values_in_u64 to be [exactly 8] or [over 8 but should be some multiple of 4] but received {}",
        values_in_u64.len()
    );
    let mut elements = from_vec(values_in_u64);
    let hash_times = if elements.len() == 8 {
        1
    } else {
        elements.len() / 4 - 1
    };
    let mut result: ElementDigest;
    if hash_times == 1 {
        result = crypto::hashers::Rp64_256::hash_elements(&elements);
    } else {
        let mut first: Vec<BaseElement> = elements.drain(0..8).collect();
        let mut to_be_hash = crypto::hashers::Rp64_256::hash_elements(&first);
        for i in 1..hash_times {
            let mut a = ElementDigest::digests_as_elements(&[to_be_hash]).to_vec();
            let mut drain_4_element: Vec<BaseElement> = elements.drain(0..4).collect();
            a.append(&mut drain_4_element);
            to_be_hash = crypto::hashers::Rp64_256::hash_elements(&a);
        }
        result = to_be_hash;
    };

    return as_u64(result).to_vec();
}

pub fn as_u64(origin: ElementDigest) -> [u64; 4] {
    let mut result = [0; 4];
    result[..1].copy_from_slice(&[origin.0[0].as_int()]);
    result[1..2].copy_from_slice(&[origin.0[1].as_int()]);
    result[2..3].copy_from_slice(&[origin.0[2].as_int()]);
    result[3..4].copy_from_slice(&[origin.0[3].as_int()]);
    result
}

pub fn rescue_two_para(para_1: [u64; 4], para_2: [u64; 4]) -> Vec<u64> {
    let mut first: Vec<u64> = para_1.to_vec();
    let mut second: Vec<u64> = para_2.to_vec();
    first.append(&mut second);
    let elements = from_vec(first);
    let result = crypto::hashers::Rp64_256::hash_elements(&elements);
    return as_u64(result).to_vec();
}

pub fn rescue_two_para_u8_32(para_1: [u64; 4], para_2: [u64; 4]) -> [u8; 32] {
    let mut first: Vec<u64> = para_1.to_vec();
    let mut second: Vec<u64> = para_2.to_vec();
    first.append(&mut second);

    let elements = from_vec(first);
    let result = crypto::hashers::Rp64_256::hash_elements(&elements);
    return result.as_bytes();
}

// convert a [u64;4] hash result to [u8;32]
#[wasm_bindgen]
pub fn to_u8array(values: String) -> Vec<u8> {
    let mut values_in_u64 = vec![];
    if values.len() != 0 {
        let values_a: Vec<&str> = values.split(',').collect();
        values_in_u64 = values_a
            .iter()
            // .map(|public_a| public_a.parse::<u64>().unwrap())
            .map(|public_a| public_a.parse::<u64>().expect("parse fail"))
            .collect();
    };
    assert!(
        (values_in_u64.len() == 4),
        "expected len of values_in_u64 to be exactly 4 but received {}",
        values_in_u64.len()
    );
    assert!(
        values_in_u64.len() == 4,
        "The input is not a U256, please check your input!"
    );
    let elements = from_vec(values_in_u64);
    return ElementDigest::new(elements.try_into().unwrap())
        .as_bytes()
        .to_vec();
}

// convert a [u8;32]
#[wasm_bindgen]
pub fn to_u64array(values: String) -> Vec<u64> {
    use utils_core::{Deserializable, SliceReader};

    let mut values_in_u8 = vec![];
    if values.len() != 0 {
        let values_a: Vec<&str> = values.split(',').collect();
        values_in_u8 = values_a
            .iter()
            // .map(|public_a| public_a.parse::<u64>().unwrap())
            .map(|public_a| public_a.parse::<u8>().expect("parse fail"))
            .collect();
    };
    assert!(
        (values_in_u8.len() == 32),
        "expected len of values_in_u8 to be exactly 32 but received {}",
        values_in_u8.len()
    );

    let mut reader = SliceReader::new(&values_in_u8);
    let convert_result = ElementDigest::read_from(&mut reader).unwrap();

    return as_u64(convert_result).to_vec();
}

/// HELPER
pub fn from_vec(origin: Vec<u64>) -> Vec<BaseElement> {
    let mut res: Vec<BaseElement> = Vec::new();
    for i in 0..origin.len() {
        res.push(math::fields::f64::BaseElement::from(origin[i as usize]))
    }
    let result = res.try_into().unwrap();
    return result;
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// GLOBAL CONSTANTS
// ================================================================================================

pub const MAX_CONTEXT_DEPTH: usize = 16;
pub const MAX_LOOP_DEPTH: usize = 8;

#[test]
fn hash_elements() {
    // let elements: [BaseElement; 8] = rand_array();
    let elements = String::from("1,2,3,4,5,6,7,8,1,2,3,4");
    let result = rescue(elements);
    let expect = [
        13731018041795440645,
        13695776054319495440,
        4543330650498719355,
        16248638019424921580,
    ];
    assert!(result == expect, "The hash result is wrong!");
}
