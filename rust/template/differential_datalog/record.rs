//! An untyped representation of DDlog values and database update commands.

use num::{ToPrimitive, BigInt, BigUint};
use std::vec;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Record {
    Bool(bool),
    Int(BigInt),
    String(String),
    Tuple(Vec<Record>),
    Array(Vec<Record>),
    PosStruct(String, Vec<Record>),
    NamedStruct(String, Vec<(String, Record)>)
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum UpdCmd {
    Insert (String, Record),
    Delete (String, Record),
    DeleteKey(String, Record)
}


#[cfg(test)]
use num::bigint::{ToBigInt, ToBigUint};

pub trait FromRecord: Sized {
    fn from_record(val: &Record) -> Result<Self, String>;
}

/// `FromRecord` trait.  For types that can be converted from cmd_parser::Record type
impl FromRecord for u8 {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Int(i) => {
                match i.to_u8() {
                    Some(x) => Result::Ok(x),
                    None    => Result::Err(format!("cannot convert {} to u8", i))
                }
            },
            v => {
                Result::Err(format!("not an int {:?}", *v))
            }
        }
    }
}

#[test]
fn test_u8() {
    assert_eq!(u8::from_record(&Record::Int(25_u8.to_bigint().unwrap())), Ok(25));
    assert_eq!(u8::from_record(&Record::Int(0xab.to_bigint().unwrap())), Ok(0xab));
    assert_eq!(u8::from_record(&Record::Int(0xabcd.to_bigint().unwrap())), Err("cannot convert 43981 to u8".to_string()));
}


impl FromRecord for u16 {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Int(i) => {
                match i.to_u16() {
                    Some(x) => Result::Ok(x),
                    None    => Result::Err(format!("cannot convert {} to u16", i))
                }
            },
            v => {
                Result::Err(format!("not an int {:?}", *v))
            }
        }
    }
}

#[test]
fn test_u16() {
    assert_eq!(u16::from_record(&Record::Int(25_u16.to_bigint().unwrap())), Ok(25));
    assert_eq!(u16::from_record(&Record::Int(0xab.to_bigint().unwrap())), Ok(0xab));
    assert_eq!(u16::from_record(&Record::Int(0xabcdef.to_bigint().unwrap())), Err("cannot convert 11259375 to u16".to_string()));
}


impl FromRecord for u32 {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Int(i) => {
                match i.to_u32() {
                    Some(x) => Result::Ok(x),
                    None    => Result::Err(format!("cannot convert {} to u32", i))
                }
            },
            v => {
                Result::Err(format!("not an int {:?}", *v))
            }
        }
    }
}

#[test]
fn test_u32() {
    assert_eq!(u32::from_record(&Record::Int(25_u32.to_bigint().unwrap())), Ok(25));
    assert_eq!(u32::from_record(&Record::Int(0xab.to_bigint().unwrap())), Ok(0xab));
    assert_eq!(u32::from_record(&Record::Int(0xabcdef.to_bigint().unwrap())), Ok(0xabcdef));
    assert_eq!(u32::from_record(&Record::Int((-0xabcdef).to_bigint().unwrap())), Err("cannot convert -11259375 to u32".to_string()));
}


impl FromRecord for u64 {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Int(i) => {
                match i.to_u64() {
                    Some(x) => Result::Ok(x),
                    None    => Result::Err(format!("cannot convert {} to u64", i))
                }
            },
            v => {
                Result::Err(format!("not an int {:?}", *v))
            }
        }
    }
}

#[test]
fn test_u64() {
    assert_eq!(u64::from_record(&Record::Int(25_u64.to_bigint().unwrap())), Ok(25));
    assert_eq!(u64::from_record(&Record::Int(0xab.to_bigint().unwrap())), Ok(0xab));
    assert_eq!(u64::from_record(&Record::Int(0xabcdef.to_bigint().unwrap())), Ok(0xabcdef));
    assert_eq!(u64::from_record(&Record::Int((-0xabcdef).to_bigint().unwrap())), Err("cannot convert -11259375 to u64".to_string()));
}

impl FromRecord for u128 {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Int(i) => {
                match i.to_u128() {
                    Some(x) => Result::Ok(x),
                    None    => Result::Err(format!("cannot convert {} to u128", i))
                }
            },
            v => {
                Result::Err(format!("not an int {:?}", *v))
            }
        }
    }
}

#[test]
fn test_u128() {
    assert_eq!(u128::from_record(&Record::Int(25_u128.to_bigint().unwrap())), Ok(25));
    assert_eq!(u128::from_record(&Record::Int(100000000000000000000000000000000000000_u128.to_bigint().unwrap())), Ok(100000000000000000000000000000000000000));
    assert_eq!(u128::from_record(&Record::Int(0xab.to_bigint().unwrap())), Ok(0xab));
    assert_eq!(u128::from_record(&Record::Int(0xabcdef.to_bigint().unwrap())), Ok(0xabcdef));
    assert_eq!(u128::from_record(&Record::Int((-0xabcdef).to_bigint().unwrap())), Err("cannot convert -11259375 to u128".to_string()));
}


impl FromRecord for BigInt {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Int(i) => Result::Ok(i.clone()),
            v => {
                Result::Err(format!("not an int {:?}", *v))
            }
        }
    }
}

#[test]
fn test_bigint() {
    let v = (-25_i64).to_bigint().unwrap();
    assert_eq!(BigInt::from_record(&Record::Int(v.clone())), Ok(v));
}

impl FromRecord for BigUint {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Int(i) => {
                match i.to_biguint() {
                    Some(x) => Result::Ok(x),
                    None    => Result::Err(format!("cannot convert {} to BigUint", i))
                }
            },
            v => {
                Result::Err(format!("not an int {:?}", *v))
            }
        }
    }
}

#[test]
fn test_biguint() {
    let vi = (25_i64).to_bigint().unwrap();
    let vu = (25_i64).to_biguint().unwrap();
    assert_eq!(BigUint::from_record(&Record::Int(vi)), Ok(vu));

    let vi = (-25_i64).to_bigint().unwrap();
    assert_eq!(BigUint::from_record(&Record::Int(vi)), Err("cannot convert -25 to BigUint".to_string()));
}


impl FromRecord for bool {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Bool(b) => { Result::Ok(*b) },
            v => {
                Result::Err(format!("not a bool {:?}", *v))
            }
        }
    }
}

#[test]
fn test_bool() {
    assert_eq!(bool::from_record(&Record::Bool(true)), Ok(true));
}


impl FromRecord for String {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::String(s) => { Result::Ok(s.clone()) },
            v => {
                Result::Err(format!("not a string {:?}", *v))
            }
        }
    }
}

#[test]
fn test_string() {
    assert_eq!(String::from_record(&Record::String("foo".to_string())), Ok("foo".to_string()));
    assert_eq!(String::from_record(&Record::Bool(true)), Err("not a string Bool(true)".to_string()));
}

impl FromRecord for () {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Tuple(args) if args.len() == 0 => {
                Ok(())
            },
            v => { Result::Err(format!("not a 0-tuple {:?}", *v)) }
        }
    }
}

macro_rules! decl_tuple_from_record {
    ( $n:tt, $( $t:tt , $i:tt),+ ) => {
        impl <$($t: FromRecord),*> FromRecord for ($($t),*) {
            fn from_record(val: &Record) -> Result<Self, String> {
                match val {
                    Record::Tuple(args) if args.len() == $n => {
                        Ok(( $($t::from_record(&args[$i])?),*))
                    },
                    v => { Result::Err(format!("not a {}-tuple {:?}", $n, *v)) }
                }
            }
        }
    };
}

decl_tuple_from_record!(2, T0, 0, T1, 1);
decl_tuple_from_record!(3, T0, 0, T1, 1, T2, 2);
decl_tuple_from_record!(4, T0, 0, T1, 1, T2, 2, T3, 3);
decl_tuple_from_record!(5, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4);
decl_tuple_from_record!(6, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5);
decl_tuple_from_record!(7, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6);
decl_tuple_from_record!(8, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7);
decl_tuple_from_record!(9, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8);
decl_tuple_from_record!(10, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9);
decl_tuple_from_record!(11, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10);
decl_tuple_from_record!(12, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11);
decl_tuple_from_record!(13, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12);
decl_tuple_from_record!(14, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12, T13, 13);
decl_tuple_from_record!(15, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12, T13, 13, T14, 14);
decl_tuple_from_record!(16, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12, T13, 13, T14, 14, T15, 15);
decl_tuple_from_record!(17, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12, T13, 13, T14, 14, T15, 15, T16, 16);
decl_tuple_from_record!(18, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12, T13, 13, T14, 14, T15, 15, T16, 16, T17, 17);
decl_tuple_from_record!(19, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12, T13, 13, T14, 14, T15, 15, T16, 16, T17, 17, T18, 18);
decl_tuple_from_record!(20, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12, T13, 13, T14, 14, T15, 15, T16, 16, T17, 17, T18, 18, T19, 19);

#[test]
fn test_tuple() {
    assert_eq!(<(bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false)])),
               Ok((true, false)));
    assert_eq!(<(bool, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false)])),
               Ok((true, false, false)));
    assert_eq!(<(bool, bool, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true)])),
               Ok((true, false, false, true)));
    assert_eq!(<(bool, bool, bool, bool, String)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string())])),
               Ok((true, false, false, true, "foo".to_string())));
    assert_eq!(<(bool, bool, bool, bool, String, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string()), Record::Bool(false)])),
               Ok((true, false, false, true, "foo".to_string(), false)));
    assert_eq!(<(bool, bool, bool, bool, String, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string()), Record::Bool(false), Record::Bool(false)])),
               Ok((true, false, false, true, "foo".to_string(), false, false)));
    assert_eq!(<(bool, bool, bool, bool, String, bool, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string()), Record::Bool(false), Record::Bool(false), Record::Bool(false)])),
               Ok((true, false, false, true, "foo".to_string(), false, false, false)));
    assert_eq!(<(bool, bool, bool, bool, String, bool, bool, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string()), Record::Bool(false), Record::Bool(false), Record::Bool(false), Record::Bool(true)])),
               Ok((true, false, false, true, "foo".to_string(), false, false, false, true)));
    assert_eq!(<(bool, bool, bool, bool, String, bool, bool, bool, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string()), Record::Bool(false), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::Bool(true)])),
               Ok((true, false, false, true, "foo".to_string(), false, false, false, true, true)));
    assert_eq!(<(bool, bool, bool, bool, String, bool, bool, bool, bool, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string()), Record::Bool(false), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::Bool(true), Record::Bool(true)])),
               Ok((true, false, false, true, "foo".to_string(), false, false, false, true, true, true)));
    assert_eq!(<(bool, bool, bool, bool, String, bool, bool, bool, bool, bool, bool, bool)>::from_record(&Record::Tuple(vec![Record::Bool(true), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::String("foo".to_string()), Record::Bool(false), Record::Bool(false), Record::Bool(false), Record::Bool(true), Record::Bool(true), Record::Bool(true), Record::Bool(false)])),
               Ok((true, false, false, true, "foo".to_string(), false, false, false, true, true, true, false)));
}


impl<T: FromRecord> FromRecord for vec::Vec<T> {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::Array(args) => {
                Result::from_iter(args.iter().map(|x| T::from_record(x)))
            },
            v => {
                T::from_record(v).map(|x|vec![x])
                //Result::Err(format!("not an array {:?}", *v))
            }
        }
    }
}

impl<K: FromRecord+Ord, V: FromRecord> FromRecord for BTreeMap<K,V> {
    fn from_record(val: &Record) -> Result<Self, String> {
        vec::Vec::from_record(val).map(|v|BTreeMap::from_iter(v))
    }
}

impl<T: FromRecord+Ord> FromRecord for BTreeSet<T> {
    fn from_record(val: &Record) -> Result<Self, String> {
        vec::Vec::from_record(val).map(|v|BTreeSet::from_iter(v))
    }
}

#[test]
fn test_vec() {
    assert_eq!(<vec::Vec<bool>>::from_record(&Record::Array(vec![Record::Bool(true), Record::Bool(false)])),
               Ok(vec![true, false]));
}

#[cfg(test)]
#[derive(Eq, PartialEq, Debug)]
struct Foo<T> {
    f1: T
}

pub fn arg_find<'a>(args: &'a Vec<(String, Record)>, argname: &str, constructor: &str) -> Result<&'a Record, String> {
    args.iter().find(|(n,_)|*n==argname).ok_or(format!("missing field {} in {}", argname, constructor)).map(|(_,v)| v)
}

#[cfg(test)]
impl <T: FromRecord> FromRecord for Foo<T> {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "Foo" if args.len() == 1 => {
                        Ok(Foo{f1: T::from_record(&args[0])?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type Foo in {:?}", c, *val))
                }
            },
            Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "Foo" if args.len() == 1 => {
                        Ok(Foo{f1: T::from_record(arg_find(args, "f1", "Foo")?)?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type Foo in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}

#[cfg(test)]
type Bbool = bool;

#[cfg(test)]
#[derive(Eq, PartialEq, Debug)]
enum DummyEnum<T> {
    Constr1{f1: Bbool, f2: String},
    Constr2{f1: T, f2: BigInt, f3: Foo<T>},
    Constr3{f1: (bool, bool)},
}

#[cfg(test)]
impl <T: FromRecord> FromRecord for DummyEnum<T> {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "Constr1" if args.len() == 2 => {
                        Ok(DummyEnum::Constr1{f1: <Bbool>::from_record(&args[0])?,
                                              f2: String::from_record(&args[1])?})
                    },
                    "Constr2" if args.len() == 3 => {
                        Ok(DummyEnum::Constr2{f1: <T>::from_record(&args[0])?,
                                              f2: <BigInt>::from_record(&args[1])?,
                                              f3: <Foo<T>>::from_record(&args[2])?})
                    },
                    "Constr3" if args.len() == 1 => {
                        Ok(DummyEnum::Constr3{f1: <(bool,bool)>::from_record(&args[0])?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type DummyEnum in {:?}", c, *val))
                }
            },
            Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "Constr1" if args.len() == 2 => {
                        Ok(DummyEnum::Constr1{f1: <Bbool>::from_record(arg_find(args, "f1", "Constr1")?)?,
                                              f2: String::from_record(arg_find(args, "f2", "Constr1")?)?})
                    },
                    "Constr2" if args.len() == 3 => {
                        Ok(DummyEnum::Constr2{f1: <T>::from_record(arg_find(args, "f1", "Constr2")?)?,
                                              f2: <BigInt>::from_record(arg_find(args, "f2", "Constr2")?)?,
                                              f3: <Foo<T>>::from_record(arg_find(args, "f3", "Constr2")?)?})
                    },
                    "Constr3" if args.len() == 1 => {
                        Ok(DummyEnum::Constr3{f1: <(bool,bool)>::from_record(arg_find(args, "f1", "Constr3")?)?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type DummyEnum in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}


#[test]
fn test_enum() {
    assert_eq!(DummyEnum::from_record(&Record::PosStruct("Constr1".to_string(),
                                                    vec![Record::Bool(true), Record::String("foo".to_string())])),
               Ok(DummyEnum::Constr1::<bool>{f1: true, f2: "foo".to_string()}));
    assert_eq!(DummyEnum::from_record(&Record::NamedStruct("Constr1".to_string(),
                                                    vec![("f1".to_string(), Record::Bool(true)), ("f2".to_string(), Record::String("foo".to_string()))])),
               Ok(DummyEnum::Constr1::<bool>{f1: true, f2: "foo".to_string()}));
    assert_eq!(DummyEnum::from_record(&Record::PosStruct("Constr2".to_string(),
                                                    vec![Record::Int((5_i64).to_bigint().unwrap()),
                                                         Record::Int((25_i64).to_bigint().unwrap()),
                                                         Record::PosStruct("Foo".to_string(), vec![Record::Int((0_i64).to_bigint().unwrap())])])),
               Ok(DummyEnum::Constr2::<u16>{f1: 5,
                                            f2: (25_i64).to_bigint().unwrap(),
                                            f3: Foo{f1: 0}}));
    assert_eq!(DummyEnum::from_record(&Record::NamedStruct("Constr2".to_string(),
                                                    vec![("f1".to_string(), Record::Int((5_i64).to_bigint().unwrap())),
                                                         ("f2".to_string(), Record::Int((25_i64).to_bigint().unwrap())),
                                                         ("f3".to_string(), Record::NamedStruct("Foo".to_string(),
                                                                                                vec![("f1".to_string(), Record::Int((0_i64).to_bigint().unwrap()))]))])),
               Ok(DummyEnum::Constr2::<u16>{f1: 5,
                                            f2: (25_i64).to_bigint().unwrap(),
                                            f3: Foo{f1: 0}}));

}
