#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_tokio_helpers_derive;
extern crate time;

use time::Timespec;
use std::collections::HashMap;
use cdrs_tokio::types::AsRustType;
use cdrs_tokio::types::value::{Bytes, Value};
use cdrs_tokio::frame::{IntoBytes, TryFromRow, TryFromUdt};
use cdrs_tokio::types::rows::Row;
use cdrs_tokio::types::udt::Udt;
use cdrs_tokio::types::list::List;
use cdrs_tokio::types::map::Map;
use cdrs_tokio::types::from_cdrs::FromCDRSByName;

// #[derive(Debug, IntoCdrsValue, TryFromRow)]
#[derive(Clone, Debug, IntoCdrsValue, TryFromRow)]
struct Udt {
    pub number: i32,
    pub number_16: i16,
    // pub vec: Vec<Vec<N>>,
    pub vec: Vec<Vec<i32>>,
    pub map: HashMap<i64, N>,
    pub opt: Option<HashMap<i64, N>>,
    pub my_timestamp: Option<Timespec>,
}

// #[derive(Debug, IntoCdrsValue, TryFromRow, TryFromUdt)]
#[derive(Clone, Debug, IntoCdrsValue, TryFromUdt)]
struct N {
    pub n: i16,
    pub x: X,
}

#[derive(Clone, Debug, IntoCdrsValue, TryFromUdt)]
struct X {
    pub n: i32,
}

fn main() {
    let udt = Udt {
        number: 12,
        number_16: 256,
        vec: vec![vec![1, 2]],
        map: HashMap::new(),
        opt: Some(HashMap::new()),
        my_timestamp: None,
    };
    let val: cdrs_tokio::types::value::Value = udt.clone().into();
    let values = query_values!(udt.clone());
    println!("as value {:?}", val);
    println!("among values {:?}", values);
}

#[cfg(test)]
mod test {
    use cdrs_tokio::query::QueryValues;
    use cdrs_tokio::types::prelude::Value;

    #[derive(DBMirror)]
    #[allow(dead_code)]
    struct SomeStruct {
        pk: i32,
        name: String,
    }

    #[test]
    fn test_insert_query() {
        assert_eq!("insert into SomeStruct(pk, name) values (?, ?)", SomeStruct::insert_query())
    }

    #[test]
    fn test_into_query_values() {
        let pk = 1;
        let name = "some name".to_string();
        let query_values: QueryValues = SomeStruct {
            pk,
            name: name.clone()
        }.into_query_values();

        if let QueryValues::NamedValues(nv) = query_values {
            assert_eq!(2, nv.len());

            let pk_val: Value = pk.into();
            assert_eq!(&pk_val, nv.get("pk").unwrap());

            let name_val: Value = name.into();
            assert_eq!(&name_val, nv.get("name").unwrap());
        } else {
            panic!("Expected named values");
        }
    }
}
