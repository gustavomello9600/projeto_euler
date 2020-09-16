fn main() {
    let (palindrome, a, b): (u32, u16, u16) = find_2_3_digit_factors_of_greatest_palindrome();
    println!("{} is the greatest palindrome that is a multiple of two three-integer numbers, namely {} and {}.", palindrome, a, b);
}


fn find_2_3_digit_factors_of_greatest_palindrome() -> (u32, u16, u16) {
    let mut diagonal: u16 = 0;

    loop {
	diagonal += 1;
	
	let a_0: u16;
	let b_0: u16;

	if diagonal % 2 == 1 {
	    let delta: u16 = (diagonal + 1)/2;
	    a_0 = 1000 - delta;
	    b_0 = 1000 - delta;
	} else {
	    let delta: u16 = diagonal/2;
	    a_0 = 1000 - delta;
            b_0 = 1000 - delta - 1;
	}

	for step in 0..1000 {
	    let a: u16 = a_0 + step;
	    if a >= 1000 {break}

	    let b: u16 = b_0 - step;
	    if b < 100 {break}

	    let product_and_factors: Option<(u32, u16, u16)> = check_if_product_is_palindrome(a, b);
	    if let Some( (product, a, b) ) = product_and_factors {
		return (product, a, b);
	    }
	}
    }
}

fn check_if_product_is_palindrome(a: u16, b: u16) -> Option<(u32, u16, u16)> {
    let product: u32 = u32::from(a) * u32::from(b);
    let digits: u8;
    
    if product >= 100000 {
	digits = 6;
    } else if product >= 10000 {
	digits = 5;
    } else if product >= 1000 {
	digits = 4;
    } else if product >= 100 {
	digits = 3;
    } else {
	return None;
    }
    
    let middle_index: u8 = if digits != 5 {digits/2} else {3};
    for i in 0..middle_index {
	if digit_in_position(i, product)
	   != digit_in_position(digits - i - 1, product) {
	    return None;
	}
    }

    return Some( (product, a, b) );
}


fn digit_in_position(i: u8, number: u32) -> u8 {
    ((number / 10_u32.pow(i as u32)) % 10) as u8
}
