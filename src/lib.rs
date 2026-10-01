#![no_std]

mod base;
use core::num;

pub use crate::base::*;

/// Numeral.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Numeral<'a>{
    value:[u8;1024],
    base:Base<'a>,
    start:usize
}

/// Numeral base.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Base<'a>{pub(crate)base_alphabet:&'a[u8]}

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
            Ok(base_str)=>base_str,
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

    /// New base from a slice of another base.
    /// 
    /// Creates a new base from the given base, its start and end.
    /// ___
    /// Neither `start` or `end` can exceed the length of the given base, 
    /// otherwise they will be clamped to the lenght of the given base.
    /// 
    /// `start` cannot be greater than `end`, otherwise they will be swapped.
    pub const fn from_base(base:Base<'a>,start:usize,end:usize)->Self{
        let base_len:usize=base.len();
        let mut start:usize=start;
        let mut end:usize=end;

        if start>base_len{start=base_len}
        if end>base_len{end=base_len}
        if start>end{(start,end)=(end,start);};

        Self{base_alphabet:base.base_alphabet.split_at(end).0.split_at(start).1}
    }

    /// New base from a slice of another base using radix.
    /// 
    /// Creates a new base from corresponding to radix base.
    pub const fn from_radix(radix:u8)->Self{
        let radix:usize=radix as usize;
        if radix<65{return Base::from_base(BASE64,0,radix)}
        else if radix<96{return Base::from_base(PRINTABLE_ASCII,0,radix)}
        Base::from_base(BASE256,0,radix)
    }
}

impl<'a>Numeral<'a>{
    pub const fn new(value:&[u8],base:Base<'a>)->Self{
        if value.is_empty()||base.len()<2{return Self{value:[0;1024],base,start:1024}}

        let value:&[u8]=trim_zeros(value,base);

        let mut counter:usize=0;
        while counter<value.len(){
            let byte:u8=value[counter];
            let mut found:bool=false;
            let mut digit_counter:usize=0;
            while digit_counter<base.base_alphabet.len() {
                if base.base_alphabet[digit_counter]==byte{
                    found=true;
                    break;
                }
                digit_counter+=1;
            }

            if!found{return Self{value:[0;1024],base,start:1024}}
            counter+=1
        }

        let mut new_value:[u8;1024]=[0u8;1024];
        let value_len:usize=value.len();

        let(start,offset)=if value_len>=1024{(0,value_len-1024)}
        else{(1024-value_len,0)};

        let mut counter:usize=0;
        while start+counter<1024{
            new_value[start+counter]=value[offset+counter];
            counter+=1;
        }

        Self{value:new_value,base,start}
    }

    pub const fn new_bin(value:&[u8])->Self{
        Numeral::new(value,BINARY)
    }

    pub const fn new_oct(value:&[u8])->Self{
        Numeral::new(value,OCTAL)
    }

    pub const fn new_dec(value:&[u8])->Self{
        Numeral::new(value,DECIMAL)
    }

    pub const fn new_hex(value:&[u8])->Self{
        Numeral::new(value,HEXADECIMAL)
    }

    pub const fn new_dec_from_u128(value:u128)->Self{
        let mut value:u128=value;
        const MAX_U128_LEN:usize=39;
        let mut result:[u8;MAX_U128_LEN]=[0;MAX_U128_LEN];
        let mut counter:usize=1;
        
        while value>0{
            result[MAX_U128_LEN-counter]=b'0'+(value%10)as u8;
            value/=10;
            counter+=1
        }
        
        let mut numeral:Numeral=Numeral::new(&result,DECIMAL);
        numeral.start=1025-counter;
        numeral
    }

    pub const fn from_raw(value:[u8;1024],base:Base<'a>,start:usize)->Self{
        let start:usize=if start>1024{1024}else{start};
        Self{value,base,start}
    }

    pub const fn from_str(value_str:&str,base_str:&'a str)->Self{
        Numeral::new(value_str.as_bytes(),Base{base_alphabet:base_str.as_bytes()})
    }

    pub const unsafe fn value_as_str_unchecked(&self)->&str{
        unsafe{core::str::from_utf8_unchecked(&self.value)}
    }

    pub const fn value_as_str(&self)->&str{
        match core::str::from_utf8(self.trimmed_value()){
            Ok(value_str)=>value_str,
            Err(_)=>""
        }
    }

    pub const fn value_as_printable_ascii(&self)->&str{
        let mut counter:usize=0;
        let value:&[u8]=self.trimmed_value();
        while counter<value.len(){
            let b:u8=value[counter];
            if b<' ' as u8||b>'~' as u8{
                return unsafe{core::str::from_utf8_unchecked(value.split_at(counter).0)}
            }
            counter+=1
        }

        unsafe{core::str::from_utf8_unchecked(value)}
    }

    const fn trimmed_value(&self)->&[u8]{
        (self.value).split_at(self.start).1
    }

    /*pub const fn convert(&self,target_base:Base)->Self{
        let source_base:Base=self.base;
        let source_radix:usize=source_base.radix();
        let target_radix:usize=target_base.radix();

        let value:&[u8]=&self.value;
        let mut result_value:[u8;1024]=[target_base.zero();1024];
    }*/
}

pub const fn trim_zeros<'a>(value:&'a[u8],base:Base)->&'a[u8]{
    if value.is_empty()||base.len()<2{return value}

    let zero:u8=base.zero();
    let mut index_counter:usize=0;

    while index_counter<value.len(){
        if value[index_counter]!=zero{return value.split_at(index_counter).1}
        index_counter+=1
    }
    value.split_at(1).0// Returns a slice containing a single zero, if the collection contained zeros only.
}