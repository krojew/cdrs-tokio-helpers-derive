#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;

use std::collections::HashMap;
use cdrs::types::AsRust;
use cdrs::types::AsRustType;
use cdrs::types::value::{Bytes, Value};
use cdrs::frame::{IntoBytes, TryFromRow, TryFromUDT};
use cdrs::types::rows::Row;
use cdrs::types::udt::UDT;
use cdrs::types::list::List;
use cdrs::types::map::Map;
use cdrs::types::from_cdrs::FromCDRSByName;

// #[derive(Debug, IntoCDRSValue, TryFromRow)]
#[derive(Clone, Debug, IntoCDRSValue, TryFromRow)]
struct Udt {
    pub number: i32,
    pub number_16: i16,
    pub vec: Vec<Vec<N>>,
    pub map: HashMap<i64, N>,
    pub opt: Option<HashMap<i64, N>>,
}

// #[derive(Debug, IntoCDRSValue, TryFromRow, TryFromUDT)]
#[derive(Clone, Debug, IntoCDRSValue, TryFromUDT)]
struct N {
    pub n: i16,
    pub x: X,
}

#[derive(Clone, Debug, IntoCDRSValue, TryFromUDT)]
struct X {
    pub n: i32,
}

fn main() {
    let udt = Udt {
        number: 12,
        number_16: 256,
        vec: vec![
            vec![
                N {
                    n: 0,
                    x: X { n: 2 },
                },
            ],
        ],
        map: HashMap::new(),
        opt: Some(HashMap::new()),
    };
    let val: Value = udt.clone().into();
    let values = query_values!(udt.clone());
    println!("as value {:?}", val);
    println!("among values {:?}", values);
}
