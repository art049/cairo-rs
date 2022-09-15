use crate::serde::deserialize_program::{
    deserialize_program, HintParams, Identifier, ReferenceManager,
};
use crate::types::errors::program_errors::ProgramError;
use crate::types::relocatable::MaybeRelocatable;
use num_bigint::BigInt;
use std::{collections::HashMap, path::Path};

#[derive(Clone)]
pub struct Program {
    pub builtins: Vec<String>,
    pub prime: BigInt,
    pub data: Vec<MaybeRelocatable>,
    pub main: Option<usize>,
    pub hints: HashMap<usize, Vec<HintParams>>,
    pub reference_manager: ReferenceManager,
    pub identifiers: HashMap<String, Identifier>,
}

impl Program {
    pub fn new(path: &Path, entrypoint: &str) -> Result<Program, ProgramError> {
        deserialize_program(path, entrypoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bigint;
    use num_traits::FromPrimitive;

    #[test]
    fn deserialize_program_test() {
        let program: Program = Program::new(
            Path::new("cairo_programs/manually_compiled/valid_program_a.json"),
            "main",
        )
        .expect("Failed to deserialize program");

        let builtins: Vec<String> = Vec::new();
        let data: Vec<MaybeRelocatable> = vec![
            MaybeRelocatable::from(BigInt::from_i64(5189976364521848832).unwrap()),
            MaybeRelocatable::from(BigInt::from_i64(1000).unwrap()),
            MaybeRelocatable::from(BigInt::from_i64(5189976364521848832).unwrap()),
            MaybeRelocatable::from(BigInt::from_i64(2000).unwrap()),
            MaybeRelocatable::from(BigInt::from_i64(5201798304953696256).unwrap()),
            MaybeRelocatable::from(BigInt::from_i64(2345108766317314046).unwrap()),
        ];

        let mut identifiers: HashMap<String, Identifier> = HashMap::new();

        identifiers.insert(
            String::from("__main__.main"),
            Identifier {
                pc: Some(0),
                type_: Some(String::from("function")),
                value: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.Args"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.ImplicitArgs"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.Return"),
            Identifier {
                pc: None,
                type_: Some(String::from("struct")),
                value: None,
            },
        );
        identifiers.insert(
            String::from("__main__.main.SIZEOF_LOCALS"),
            Identifier {
                pc: None,
                type_: Some(String::from("const")),
                value: Some(bigint!(0)),
            },
        );

        assert_eq!(
            program.prime,
            BigInt::parse_bytes(
                b"3618502788666131213697322783095070105623107215331596699973092056135872020481",
                10
            )
            .unwrap()
        );
        assert_eq!(program.builtins, builtins);
        assert_eq!(program.data, data);
        assert_eq!(program.main, Some(0));
        assert_eq!(program.identifiers, identifiers);
    }
}
