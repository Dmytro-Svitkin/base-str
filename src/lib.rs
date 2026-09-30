mod base;
pub use crate::base::*;

struct Numeral<'a>{
    value:&'a[u8],
    base:Base<'a>
}

struct Base<'a>{alphabet:&'a[u8]}

pub const fn str_to_base(str_alphabet:&str)->&[u8]{
    str_alphabet.as_bytes()
}

pub const fn unsafe_base_to_str(base:&[u8])->&str{
    unsafe{core::str::from_utf8_unchecked(base)}
}

/*pub const fn base_to_str(base:&[u8])->&str{
    unsafe{core::str::from_utf8_unchecked(base)}
}*/

pub const fn base<'a>(x:u8)->&'a[u8]{// Limited to u8 and redeclared to usize, because I want to make bases over 255 impossible. Note that base(256) is not possible (but possible via constant BASE256).
    let x:usize=x as usize;
    if x<63{return ALPHANUMERIC.split_at(x).0}// Might replace that by a match with miscilinious bases (e.g., MORSE) added.
    else if x<96{return ASCII.split_at(x).0}
    BASE256.split_at(x).0
}

/// Derrive a smaller or equal base from an old base by giving the number of digits.
pub const fn new_base(old_base:&[u8],digits:u8)->&[u8]{
    let digits:usize=digits as usize;
    if old_base.len()>digits{return BASE256.split_at(digits).0}
    old_base.split_at(digits).0
    
}

const fn find_digit(base:&[u8],digit:u8)->usize{
    let mut counter:usize=0;
    
    while counter<base.len(){
        if base[counter]==digit{return counter}
        counter+=1
    }
    panic!("[!] INVALID DIGIT")
}

pub const fn trim_zeros<'a>(value:&'a[u8],base:&[u8])->&'a[u8]{
    if value.is_empty()||base.len()<2{return value}

    let zero:u8=base[0];
    let mut index_counter:usize=0;

    while index_counter<value.len(){
        if value[index_counter]!=zero{return value.split_at(index_counter).1}
        index_counter+=1
    }
    value.split_at(1).0// Returns a slice containing a single zero, if the collection contained zeros only.
}

pub const fn const_convert(value:&[u8],source_base:&[u8],target_base:&[u8],out_buf:&mut[u8])->usize{
    let source_base_len:usize=source_base.len();
    let target_base_len:usize=target_base.len();

    if value.is_empty()||source_base_len==0||target_base_len==0{return 0};

    let value:&[u8]=trim_zeros(value, source_base);
    if value.is_empty(){return 0}

    const MAX_LEN:usize=512;
    if value.len()>MAX_LEN{panic!("[!] INPUT VALUE IS TOO LARGE")}

    let mut source_digits:[usize;512]=[0usize;MAX_LEN];
    let mut counter:usize=0;
    while counter<value.len(){
        source_digits[counter]=find_digit(source_base,value[counter]);
        counter+=1
    }

    let mut target_indices:[usize;512]=[0usize;MAX_LEN];
    let mut target_count:usize=0;
    let mut start:usize=0;

    while start<value.len(){
        let mut current_carry:usize=0;
        let mut new_start:usize=start;
        let mut leading_zero:bool=true;

        let mut ix:usize=start;
        while ix<value.len(){
            let working_value:usize=current_carry*source_base_len+source_digits[ix];
            let quotient_digit:usize=working_value/target_base_len;
            current_carry=working_value%target_base_len;
            source_digits[ix]=quotient_digit;

            if leading_zero{
                if quotient_digit==0{new_start+=1}
                else{leading_zero=false}
            }
            ix+=1
        }

        start=new_start;

        if target_count>=out_buf.len()||target_count>=MAX_LEN{panic!("Output buffer overflow")}
        target_indices[target_count]=current_carry;
        target_count+=1;
    }

    let mut write_idx:usize=0;

    while write_idx<target_count{
        let digit_idx:usize=target_indices[target_count-1-write_idx];
        out_buf[write_idx]=target_base[digit_idx];
        write_idx+=1;
    }

    target_count
}