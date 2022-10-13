/* 
* Module to big number multiplication
*
* */ 

pub struct BigNumber { }

// Time complexity to karatsuba O(n^{1.59})
// is the best multiplication algorithms to big number
// it use a divide and conquer approach and base arithmetical math
fn mult_karatsuba(num_a: BigNumber, num_b: BigNumber) -> BigNumber{
}



fn main() {
    // # TODO  fix it to create a big number with i8 in base case
    // # TODO this should be a linked list 1 -> 3 -> 1 -> 2 -> 3 -> ... -> 1
    // then we need to apply karatsuba multiplication to linked list
    let num_a: BigNumber = BigNumber::new(131231232131142314321423421342142141234124123412421)
    let num_b: BigNumber = BigNumber::new(31231232131142314321423421342142141234124123412421)

    mult_karatsuba(1, 2);
}

