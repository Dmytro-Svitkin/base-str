use u8_base_converter::*;

#[test]
fn test_base(){
    assert_eq!(Base::new(b"12345"),Base::from_str("12345"));
    println!("{:?}",Base::new(b"+-="));
    assert_eq!("abc",Base::new(b"abc").as_printable_ascii());
    assert_eq!(DECIMAL,Base::from_radix(10));
    assert_eq!(HEXADECIMAL.len(),16)
}

