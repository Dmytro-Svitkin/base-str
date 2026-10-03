#![no_std]

mod base;
pub use crate::base::*;

/// A numeral.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Numeral<'a>{
    value:[u8;1024],
    base:Base<'a>,
    start:usize
}

/// A numeral base.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Base<'a>{pub(crate)base_alphabet:&'a[u8]}

impl<'a>Base<'a>{
    /// New base.
    /// 
    /// Creates a new base from the given slice of byte array (`&[u8]`).
    pub const fn new(base_alphabet:&'a[u8])->Self{
        Self{base_alphabet}
    }

    /// Length of the base.
    /// 
    /// Returns the number of digits of the base.
    pub const fn len(&self)->usize{
        self.base_alphabet.len()
    }

    /// Radix of the base.
    /// 
    /// Returns the radix of the base.
    pub const fn radix(&self)->usize{
        self.len()
    }

    /// Relative (to base) *zero* (i.e., first) digit.
    /// 
    /// Returns the first digit (i.e., *zero*) byte (`u8`) of the base.
    pub const fn zero(&self)->u8{
        self.base_alphabet[0]
    }

    /// Base to unsafe string slice (`&str`) conversion.
    /// 
    /// Does not verify if the base contains valid UTF-8 characters.
    /// 
    /// Returns a string slice (`&str`) from the given base.
    /// ___
    /// ## Safety
    /// The bytes passed in must be valid UTF-8.
    /// ___
    pub const unsafe fn to_str_unchecked(&self)->&str{
        unsafe{core::str::from_utf8_unchecked(self.base_alphabet)}
    }

    /// Base to string slice (`&str`) conversion.
    /// 
    /// Returns a string slice (`&str`) from the given base.
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
    /// Creates a new base from corresponding to the radix given (e.g., `base(16)` is hexadecimal).
    pub const fn from_radix(radix:u8)->Self{
        let radix:usize=radix as usize;
        if radix<65{return Base::from_base(BASE64,0,radix)}
        else if radix<96{return Base::from_base(PRINTABLE_ASCII,0,radix)}
        Base::from_base(BASE256,0,radix)
    }
}

impl<'a>Numeral<'a>{
    /// Length of the numeral.
    /// 
    /// Returns the number of digits of the numeral.
    pub const fn len(&self)->usize{
        1024-self.start
    }

    /// Radix of the numeral's base.
    /// 
    /// Returns the radix of the numeral's base.
    pub const fn radix(&self)->usize{
        self.base.radix()
    }

    /// Base of the numeral.
    /// 
    /// Returns the base of the numeral.
    pub const fn base(&self)->Base<'a>{
        Base{base_alphabet:self.base.base_alphabet}
    }

    /// New numeral.
    /// 
    /// Creates a numeral based on the given value bytes slice (`&[u8]`) and its base, with zeros being trimmed.
    /// 
    /// Returns an *empty* numeral in the case of value containing an *invalid digit*.
    pub const fn new(value:&[u8],base:Base<'a>)->Self{
        let value:&[u8]=trim_zeros(value,base);
        let value_len:usize=value.len();
        let mut new_value:[u8;1024]=[base.zero();1024];

        let mut value_counter:usize=0;

        while value_counter<value_len{
            let mut legal_digit:bool=false;
            let mut base_counter:usize=0;

            while base_counter<base.len(){
                if base.base_alphabet[base_counter]==value[value_counter]{legal_digit=true;break}
                base_counter+=1
            }

            if legal_digit==false{return Self{value:new_value,base,start:1024}}
            value_counter+=1
        }

        if value_len==0{return Self{value:new_value,base,start:2024}}

        let offset:usize=1024-value_len;
        let mut counter:usize=0;

        while counter<value_len{
            new_value[offset+counter]=value[counter];
            counter+=1
        }

        Self{value:new_value,base,start:offset}
    }

    /// New *binary* numeral.
    /// 
    /// Creates a *binary* numeral based on the given value bytes slice (`&[u8]`).
    /// 
    /// Returns an *empty* numeral in the case of value containing an *invalid digit*.
    pub const fn new_bin(value:&[u8])->Self{
        Numeral::new(value,BINARY)
    }

    /// New *octal* numeral.
    /// 
    /// Creates a *octal* numeral based on the given value bytes slice (`&[u8]`).
    /// 
    /// Returns an *empty* numeral in the case of value containing an *invalid digit*.
    pub const fn new_oct(value:&[u8])->Self{
        Numeral::new(value,OCTAL)
    }

    /// New *decimal* numeral.
    /// 
    /// Creates a *decimal* numeral based on the given value bytes slice (`&[u8]`).
    /// 
    /// Returns an *empty* numeral in the case of value containing an *invalid digit*.
    pub const fn new_dec(value:&[u8])->Self{
        Numeral::new(value,DECIMAL)
    }

    /// New *hexadecimal* numeral.
    /// 
    /// Creates a *hexadecimal* numeral based on the given value bytes slice (`&[u8]`).
    /// 
    /// Returns an *empty* numeral in the case of value containing an *invalid digit*.
    pub const fn new_hex(value:&[u8])->Self{
        Numeral::new(value,HEXADECIMAL)
    }

    /// New *decimal* numeral from `u128`.
    /// 
    /// Creates a *decimal* numeral based on the given raw value in `u128`.
    pub const fn new_dec_from_u128(value:u128)->Self{
        let mut value:u128=value;
        const MAX_U128_LEN:usize=39;
        let decimal_zero:u8=DECIMAL.zero();
        let mut result:[u8;MAX_U128_LEN]=[decimal_zero;MAX_U128_LEN];
        let mut counter:usize=1;
        
        while value>0{
            result[MAX_U128_LEN-counter]=decimal_zero+(value%10)as u8;
            value/=10;
            counter+=1
        }
        
        Numeral::new(trim_zeros(&result,DECIMAL),DECIMAL)
    }

    /*
    pub const fn from_raw(value:[u8;1024],base:Base<'a>,start:usize)->Self{
        let start:usize=if start>1024{1024}else{start};
        Self{value,base,start}
    }
    */

    /// New numeral.
    /// 
    /// Creates a numeral based on the given value string slice (`&str`) and its base string slice (`&str`), with zeros being trimmed.
    /// 
    /// Returns an *empty* numeral in the case of value containing an *invalid digit*.
    pub const fn from_str(value_str:&str,base_str:&'a str)->Self{
        Numeral::new(value_str.as_bytes(),Base{base_alphabet:base_str.as_bytes()})
    }

    /// Numeral to unsafe string slice (`&str`) conversion.
    /// 
    /// Does not verify if the numeral contains valid UTF-8 characters.
    /// 
    /// Returns a string slice (`&str`) from the given numeral.
    /// ___
    /// ## Safety
    /// The bytes passed in must be valid UTF-8.
    /// ___
    pub const unsafe fn value_as_str_unchecked(&self)->&str{
        unsafe{core::str::from_utf8_unchecked(&self.value)}
    }

    /// Numeral to string slice (`&str`) conversion.
    /// 
    /// Returns a string slice (`&str`) from the given numeral.
    pub const fn value_as_str(&self)->&str{
        match core::str::from_utf8(self.trimmed_value()){
            Ok(value_str)=>value_str,
            Err(_)=>""
        }
    }

    /// Base to printable ASCII (`&str`) conversion.
    /// 
    /// Returns a `&str` slice containing printable ASCII characters up to the first non-printable character.
    pub const fn value_as_printable_ascii(&self)->&str{
        let mut counter:usize=0;
        let value:&[u8]=self.trimmed_value();
        let (printable_ascii_zero,printable_ascii_max)=(PRINTABLE_ASCII.zero(),PRINTABLE_ASCII.base_alphabet[PRINTABLE_ASCII.len()-1]);
        while counter<value.len(){
            let b:u8=value[counter];
            if b<printable_ascii_zero||b>printable_ascii_max{
                return unsafe{core::str::from_utf8_unchecked(value.split_at(counter).0)}
            }
            counter+=1
        }

        unsafe{core::str::from_utf8_unchecked(value)}
    }

    const fn trimmed_value(&self)->&[u8]{
        (self.value).split_at(self.start).1
    }

    /// Empty numeral check.
    /// 
    /// Returns `true` if the numeral is empty, returns `false` otherwise.
    pub const fn is_empty(&self)->bool{
        self.start>1023
    }


    pub const fn converted_to(&self,target_base:Base<'a>)->Self{
        let mut source_value:[u8;1024]=self.value;
        //let source_value:&mut[u8]=source_value.split_at_mut(self.start).1;

        let mut target_value:[u8;1024]=[target_base.zero();1024];
        let mut target_start:usize=1024;

        let source_base:Base=self.base;
        let source_base_radix:usize=source_base.radix();
        let target_base_radix:usize=target_base.radix();

        if source_value.is_empty()||source_base_radix==0||target_base_radix==0{
            return Self{value:target_value,base:target_base,start:1024}
        }

        loop{
            let mut carry:usize=0;
            let mut all_zero:bool=true;
            let mut value_counter:usize=self.start;

            while value_counter<1024{
                let digit_val:usize=digit_ix(source_value[value_counter],source_base);
                let rcl:usize=carry*source_base_radix+digit_val;

                let quotient:usize=rcl/target_base_radix;
                carry=rcl%target_base_radix;

                source_value[value_counter]=source_base.base_alphabet[quotient];

                if quotient>0{all_zero=false}
                value_counter+=1
            }

            if target_start==0{break}

            target_start-=1;
            target_value[target_start]=target_base.base_alphabet[carry];

            if all_zero{break}
        }

        Self{value:target_value,base:target_base,start:target_start}
    }
}

const fn trim_zeros<'a>(value:&'a[u8],base:Base)->&'a[u8]{
    let base_radix:usize=base.radix();

    if value.is_empty()||base_radix==1{return value}
    else if base_radix==0{return value.split_at(0).0}

    let zero:u8=base.zero();
    let mut value_counter:usize=0;

    while value_counter<value.len(){
        if value[value_counter]!=zero{return value.split_at(value_counter).1}
        value_counter+=1
    }
    value.split_at(1).0// Returns a slice containing a single zero, if the collection contained zeros only.
}

const fn digit_ix(digit:u8,base:Base)->usize{
    let base_len:usize=base.len();
    let mut index_counter:usize=0;

    while index_counter<base_len{
        if digit==base.base_alphabet[index_counter]{return index_counter}
        index_counter+=1
    }
    0
}