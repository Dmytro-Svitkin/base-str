#![no_std]

mod base;
pub use crate::base::*;

struct Numeral<'a>{
    value:&'a[u8],
    base:Base<'a>
}

/// Numeral base.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Base<'a>{base_alphabet:&'a[u8]}

impl<'a>Base<'a>{
    /// New base.
    /// 
    /// Creates a new base from the given slice of byte array.
    pub const fn new(base_alphabet:&'a[u8])->Self{
        Self{base_alphabet}
    }

    /// Length of base.
    /// 
    /// Returns the number of digits of the base.
    pub const fn len(&self)->usize{
        self.base_alphabet.len()
    }

    /// Radix of base.
    /// 
    /// Returns the radix of the base.
    pub const fn radix(&self)->usize{
        self.len()
    }

    /// Relative (to base) zero digit.
    /// 
    /// Returns the first digit of the base.
    pub const fn zero(&self)->u8{
        self.base_alphabet[0]
    }

    /// Base to unsafe `&str` conversion.
    /// 
    /// Does not verify if the base contains valid UTF-8 characters.
    /// 
    /// Returns `&str`.
    /// ___
    /// ## Safety
    /// The bytes passed in must be valid UTF-8.
    /// ___
    pub const unsafe fn to_str_unchecked(&self)->&str{
        unsafe{core::str::from_utf8_unchecked(self.base_alphabet)}
    }

    /// Base to `&str` conversion.
    /// 
    /// Returns `&str`.
    pub const fn as_str(&self)->&str{
        match core::str::from_utf8(self.base_alphabet){
            Ok(str_base)=>str_base,
            Err(_)=>""
        }
    }

    /// Base to printable ASCII (`&str`) conversion.
    /// 
    /// Returns a `&str` slice containing printable ASCII characters up to the first non-printable character.
    pub const fn as_printable_ascii(&self)->&str{
        let mut counter:usize=0;
        
        while counter<self.base_alphabet.len(){
            let b:u8=self.base_alphabet[counter];
            if b<' ' as u8||b>'~' as u8{
                let valid_prefix:&[u8]=self.base_alphabet.split_at(counter).0;
                return unsafe{core::str::from_utf8_unchecked(valid_prefix)}
            }
            counter+=1
        }

        unsafe{core::str::from_utf8_unchecked(self.base_alphabet)}
    }

    /// New base from base string slice (`&str`).
    /// 
    /// Creates a new base from the given slice of text (`&str`).
    pub const fn from_str(base_alphabet_str:&'a str)->Self{
        Self{base_alphabet:base_alphabet_str.as_bytes()}
    }
}

impl<'a>Numeral<'a>{
    
}