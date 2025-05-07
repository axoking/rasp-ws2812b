// New method to convert byte to spi bytes
pub fn byte_to_spi_bytes(byte: u8) -> [u8; 3] {
	[
		0b00100100 + (byte<<1&0b10) + (byte<<3&0b10000) + (byte<<5&0b10000000),
		0b01001001 + (byte>>1&0b100) + (byte<<1&0b100000),
		0b10010010 + (byte>>5&0b1) + (byte>>3&0b1000) + (byte>>1&0b1000000)
	]
}

// Convert panel bits into their SPI counterparts
// 0 -> 001
// 1 -> 011
pub fn byte_to_spi_bytes_old(input: u8) -> [u8; 3] {
    // first convert the u8 to 24 bits
    let mut bool_array = [false; 24];
    for bit_index in 0..8 {
        let bit = input & (1 << bit_index) != 0;
        let out_index = bit_index * 3;

        // first bit is always 0
        // this could be omitted because the array is initialized to false
        bool_array[out_index] = false;

        bool_array[out_index + 1] = bit;

        // last bit is always 1
        bool_array[out_index + 2] = true;
    }

    // then convert the 24 bits to three u8
    [
        bool_slice_to_u8(&bool_array[0..8]),
        bool_slice_to_u8(&bool_array[8..16]),
        bool_slice_to_u8(&bool_array[16..24]),
    ]
}

fn bool_slice_to_u8(input: &[bool]) -> u8 {
    if input.len() != 8 { panic!("bool to u8 conversion requires exactly 8 booleans") }

    let mut out = 0b0000_0000u8;

    for (carry_bit, flag) in input.iter().enumerate() {
        if *flag { out += 0b0000_0001u8 << carry_bit }
    }

    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn convert_0() {
        let input = 0;
        let output = super::byte_to_spi_bytes(input);

        assert_eq!([36, 73, 146], output);
    }

    #[test]
    fn convert_1() {
        let input = 1;
        let output = super::byte_to_spi_bytes(input);

        assert_eq!([38, 73, 146], output);
    }

    #[test]
    fn convert_255() {
        let input = 255;
        let output = super::byte_to_spi_bytes(input);

        assert_eq!([182, 109, 219], output);
    }
}
