
use serde::{Deserialize, Serialize};
use num_bigint::BigUint;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BSGS(String);


use num_traits::{One, Zero};
use std::collections::HashMap;
// https://github.com/ashutosh1206/Crypton/blob/master/Discrete-Logarithm-Problem/Algo-Baby-Step-Giant-Step/bsgs.py




impl BSGS {
    // """
    // Reference:

    // To solve DLP: h = g^x % p and get the value of x.
    // We use the property that x = i*m + j, where m = ceil(sqrt(n))

    // :parameters:
    //     g : int/long
    //             Generator of the group
    //     h : int/long
    //             Result of g**x % p
    //     p : int/long
    //             Group over which DLP is generated. Commonly p is a prime number

    // :variables:
    //     m : int/long
    //             Upper limit of baby steps / giant steps
    //     x_poss : int/long
    //             Values calculated in each giant step
    //     c : int/long
    //             Giant Step pre-computation: c = g^(-m) % p
    //     i, j : int/long
    //             Giant Step, Baby Step variables
    //     lookup_table: dictionary
    //             Dictionary storing all the values computed in the baby step
    // """
    pub fn new(bsgs: String) -> Self {
        Self(bsgs)
    }


    impl Secp256k1 {
        // TODO
    }
    


    pub fn run(g: &BigUint, h: &BigUint, p: &BigUint) -> Option<BigUint>  {
        let mod_size = p.bits();

        println!("[+] Using BSGS algorithm to solve DLP");
        println!("[+] Modulus size: {}. Warning! BSGS not space efficient\n", mod_size);
    
        let m = (*&p - BigUint::one()).sqrt() + BigUint::one();
        let mut lookup_table: HashMap<BigUint, BigUint> = HashMap::new();
    
        // Baby Step
        let mut j = BigUint::zero();
        while &j < &m {
            let key = g.modpow(&j, &p);
            lookup_table.insert(key.clone(), j.clone());
            j += BigUint::one();
        }

    
        // Giant Step pre-computation
        let c = g.modpow(&(&m * (*&p - BigUint::from(2u32))), &p);
    
        // Giant Steps
        let mut i = BigUint::zero();
        while &i < &m {
            let temp = &(h * &c.modpow(&i, &p)) % p;
            if let Some(j) = lookup_table.get(&temp) {
                // x found
                return Some(i * &m + j);
            }
            i += BigUint::one();
        }
    
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn it_works() {
        let answer = BigUint::parse_bytes(b"4178319614", 10).unwrap();
        let g = BigUint::parse_bytes(b"2", 10).unwrap();
        let h = BigUint::parse_bytes(b"4178319614", 10).unwrap();
        let p = BigUint::parse_bytes(b"6971096459", 10).unwrap();
        let result = BSGS::run(&g, &h, &p);
        let c =  g.modpow(&result.unwrap(), &p);
        assert_eq!(c, answer);
    }

    #[test]
    fn it_works_2() {
        let answer = BigUint::parse_bytes(b"362073897", 10).unwrap();
        let g = BigUint::parse_bytes(b"3", 10).unwrap();
        let h = BigUint::parse_bytes(b"362073897", 10).unwrap();
        let p = BigUint::parse_bytes(b"2500000001", 10).unwrap();
        let result = BSGS::run(&g, &h, &p);
        let c =  g.modpow(&result.unwrap(), &p);
        assert_eq!(c, answer);
    }
}
