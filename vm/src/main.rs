use serde::{Deserialize, Serialize};
use transform::{from_bytes, BytesSerializer};

use std::collections::HashMap;
pub fn main() {
    let ser = BytesSerializer::new();

    // This will fail because we don't support String
    let test_string = "hello".to_string();
    let result = ser.to_bytes(&test_string);
    assert!(result.is_err());

    // This will succeed because we support Option
    let test_option: Option<u8> = Some(0);
    let result = ser.to_bytes(&test_option);
    assert!(result.is_ok());
    let back: Option<u8> = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_option);

    // This will succeed because we support Unit
    let test_unit = ();
    let result = ser.to_bytes(&test_unit);
    assert!(result.is_ok());
    let back: () = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_unit);

    // This will fail because we don't support Unit Struct
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct UnitStruct;

    let test_unit_struct = UnitStruct;
    let result = ser.to_bytes(&test_unit_struct);
    assert!(result.is_ok());
    let back: UnitStruct = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_unit_struct);

    // This will fail because we don't support Unit Variant
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum UnitVariant {
        A,
    }

    let test_unit_variant = UnitVariant::A;
    let result = ser.to_bytes(&test_unit_variant);
    assert!(result.is_ok());
    let back: UnitVariant = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_unit_variant);

    // This will fail because we don't support Newtype Struct
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct NewType(u32);

    let test_newtype_struct = NewType(0);
    let result = ser.to_bytes(&test_newtype_struct);
    assert!(result.is_ok());
    let back: NewType = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_newtype_struct);

    // This will fail because we don't support Newtype Variant
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum NewTypeVariant {
        A(u32),
    }

    let test_newtype_variant = NewTypeVariant::A(0);
    let result = ser.to_bytes(&test_newtype_variant);
    assert!(result.is_ok());
    let back: NewTypeVariant = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_newtype_variant);

    // This will succeed because we support Vec for u8 & u32
    let test_vec: Vec<u32> = vec![0, 1, 2, 3];
    let result = ser.to_bytes(&test_vec);
    assert!(result.is_ok());
    println!("{:?}", result);
    let back: Vec<u32> = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_vec);

    // This will succeed because we support Tuple for u8 & u32
    let test_tuple: (u8, u32) = (0, 1);
    let result = ser.to_bytes(&test_tuple);
    assert!(result.is_ok());
    let back: (u8, u32) = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_tuple);

    // This will succeed because we support Tuple Struct for u8 & u32
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TupleStruct(u8, u32);

    let test_tuple_struct = TupleStruct(0, 1);
    let result = ser.to_bytes(&test_tuple_struct);
    assert!(result.is_ok());
    let back: TupleStruct = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_tuple_struct);

    // This will fail because we don't support Tuple Variant
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum TupleVariant {
        B,          // But we don't support variant itself
        A(u8, u32), // We support tuple in variant
    }

    let test_tuple_variant = TupleVariant::A(0, 1);
    let result = ser.to_bytes(&test_tuple_variant);
    assert!(result.is_ok());
    let back: TupleVariant = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_tuple_variant);

    let test_tuple_variant = TupleVariant::B;
    let result = ser.to_bytes(&test_tuple_variant);
    assert!(result.is_ok());
    let back: TupleVariant = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_tuple_variant);

    // This will fail because we don't support Map for u8 & u32
    let mut test_map = HashMap::new();
    test_map.insert(0 as u8, 1 as u8);
    let result = ser.to_bytes(&test_map);
    assert!(result.is_err());

    // This will succeed because we support Struct for u8 & u32
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Struct {
        a: u8,
        b: u32,
    }

    let test_struct = Struct { a: 0, b: 1 };
    let result = ser.to_bytes(&test_struct);
    assert!(result.is_ok());
    let back: Struct = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_struct);

    // This will fail because we don't support Struct Variant
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum StructVariant {
        A { a: u8, b: u32 }, // We support struct in variant
        B,                   // But we don't support variant itself
    }

    let test_struct_variant = StructVariant::A { a: 0, b: 1 };
    let result = ser.to_bytes(&test_struct_variant);
    assert!(result.is_ok());
    let back: StructVariant = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_struct_variant);

    let test_struct_variant = StructVariant::B;
    let result = ser.to_bytes(&test_struct_variant);
    assert!(result.is_ok());
    let back: StructVariant = from_bytes(result.unwrap()).unwrap();
    assert_eq!(back, test_struct_variant);
}
