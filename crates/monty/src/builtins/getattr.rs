//! Implementation of the getattr() builtin function.

use crate::{
    ExcType,
    args::ArgValues,
    exception_private::{RunResult, SimpleException},
    heap::Heap,
    intern::Interns,
    resource::ResourceTracker,
    value::Value,
};

/// Implementation of the getattr() builtin function.
///
/// Returns the value of the named attribute of an object
/// If the attribute doesn't exist and a default is provided, returns the default
/// If no default is provided and the attribute doesn't exist, raises AttributeError
///
/// Note: name must be a string. Per Python docs, "Since private name mangling happens
/// at compilation time, one must manually mangle a private attribute's (attributes with
/// two leading underscores) name in order to retrieve it with getattr()."
///
/// Examples:
/// ```python
/// getattr(obj, 'x')             # Get obj.x
/// getattr(obj, 'y', None)       # Get obj.y or None if not found
/// getattr(module, 'function')   # Get module.function
/// ```
pub fn builtin_getattr(heap: &mut Heap<impl ResourceTracker>, args: ArgValues, interns: &Interns) -> RunResult<Value> {
    let (mut positional, kwargs) = args.into_parts();

    let pos_count = positional.len();
    let kw_count = kwargs.len();

    // Check for unsupported kwargs
    if !kwargs.is_empty() {
        for (k, v) in kwargs {
            k.drop_with_heap(heap);
            v.drop_with_heap(heap);
        }
        for v in positional {
            v.drop_with_heap(heap);
        }
        return Err(ExcType::type_error_arg_count("getattr", 2, pos_count + kw_count));
    }

    if !(2..=3).contains(&pos_count) {
        for v in positional {
            v.drop_with_heap(heap);
        }
        return Err(ExcType::type_error_arg_count("getattr", 2, pos_count));
    }

    let object = positional.next().expect("positional must have 2 or 3 arguments");
    let name = positional.next().expect("positional must have 2 or 3 arguments");
    let default = positional.next();

    let Value::InternString(name_id) = name else {
        object.drop_with_heap(heap);
        name.drop_with_heap(heap);
        if let Some(d) = default {
            d.drop_with_heap(heap);
        }
        return Err(SimpleException::new_msg(ExcType::TypeError, "getattr(): attribute name must be string").into());
    };

    name.drop_with_heap(heap);

    match object.py_get_attr(name_id, heap, interns) {
        Ok(value) => {
            object.drop_with_heap(heap);

            if let Some(d) = default {
                d.drop_with_heap(heap);
            }

            Ok(value)
        }
        Err(_) if default.is_some() => {
            object.drop_with_heap(heap);
            Ok(default.unwrap())
        }
        Err(e) => {
            object.drop_with_heap(heap);
            if let Some(d) = default {
                d.drop_with_heap(heap);
            }

            Err(e)
        }
    }
}
