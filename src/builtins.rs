/// Built-in functions for the Python interpreter.
///
/// This module contains the `Builtins` enum representing all supported built-in
/// functions (print, len, str, etc.).
use strum::{AsRefStr, Display, EnumString};

use crate::args::ArgObjects;
use crate::exceptions::{exc_err_fmt, ExcType};
use crate::heap::{Heap, HeapData};
use crate::object::Object;
use crate::run::RunResult;
use crate::values::PyValue;

/// Enumerates every interpreter-native Python builtin Monty currently supports.
///
/// Uses strum derives for automatic `Display`, `FromStr`, and `AsRef<str>` implementations.
/// All variants serialize to lowercase (e.g., `Print` -> "print").
#[derive(Debug, Clone, Copy, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Builtins {
    Print,
    Len,
    Str,
    Repr,
    Id,
    Range,
    Hash,
}

impl Builtins {
    /// Executes the builtin with the provided positional arguments.
    pub fn call<'c, 'e>(self, heap: &mut Heap<'c, 'e>, args: ArgObjects<'c, 'e>) -> RunResult<'c, Object<'c, 'e>> {
        match self {
            Self::Print => {
                match args {
                    ArgObjects::Zero => {}
                    ArgObjects::One(a) => println!("{}", a.py_str(heap)),
                    ArgObjects::Two(a1, a2) => println!("{} {}", a1.py_str(heap), a2.py_str(heap)),
                    ArgObjects::Many(args) => {
                        let mut iter = args.iter();
                        print!("{}", iter.next().unwrap().py_str(heap));
                        for object in iter {
                            print!(" {}", object.py_str(heap));
                        }
                        println!();
                    }
                }
                Ok(Object::None)
            }
            Self::Len => {
                let object = args.get_one_arg("len")?;
                match object.py_len(heap) {
                    Some(len) => Ok(Object::Int(len as i64)),
                    None => exc_err_fmt!(ExcType::TypeError; "Object of type {} has no len()", object.py_repr(heap)),
                }
            }
            Self::Str => {
                let object = args.get_one_arg("str")?;
                let object_id = heap.allocate(HeapData::Str(object.py_str(heap).into_owned().into()));
                Ok(Object::Ref(object_id))
            }
            Self::Repr => {
                let object = args.get_one_arg("repr")?;
                let object_id = heap.allocate(HeapData::Str(object.py_repr(heap).into_owned().into()));
                Ok(Object::Ref(object_id))
            }
            Self::Id => {
                let mut object = args.get_one_arg("id")?;
                let id = object.id(heap);
                // TODO might need to use bigint here
                Ok(Object::Int(id as i64))
            }
            Self::Range => {
                let object = args.get_one_arg("range")?;
                let size = object.as_int()?;
                Ok(Object::Range(size))
            }
            Self::Hash => {
                let object = args.get_one_arg("hash")?;
                match object.py_hash_u64(heap) {
                    Some(hash) => Ok(Object::Int(hash as i64)),
                    None => Err(ExcType::type_error_unhashable(object.py_type(heap))),
                }
            }
        }
    }
}
