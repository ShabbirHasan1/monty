//! Implementation of the setattr() builtin function.

use crate::{
    ExcType,
    args::ArgValues,
    exception_private::{RunResult, SimpleException},
    heap::Heap,
    intern::Interns,
    resource::ResourceTracker,
    value::Value,
};

/// Implementation of the setattr() builtin function.
///
/// Sets the named attribute on the given object to the specified value
/// This is the counterpart to getattr(). Returns None on success
///
/// Note: Currently only dataclass objects support attribute setting
/// Other object types will raise AttributeError
///
/// Examples:
/// ```python
/// setattr(obj, 'x', 42)      # Set obj.x = 42
/// setattr(obj, 'name', 'foo') # Set obj.name = 'foo'
/// ```
pub fn builtin_setattr(heap: &mut Heap<impl ResourceTracker>, args: ArgValues, interns: &Interns) -> RunResult<Value> {
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
        return Err(ExcType::type_error_arg_count("setattr", 3, pos_count + kw_count));
    }

    if pos_count != 3 {
        for v in positional {
            v.drop_with_heap(heap);
        }
        return Err(ExcType::type_error_arg_count("setattr", 3, pos_count));
    }

    let object = positional.next().unwrap();
    let name = positional.next().unwrap();
    let value = positional.next().unwrap();

    let Value::InternString(name_id) = name else {
        object.drop_with_heap(heap);
        name.drop_with_heap(heap);
        value.drop_with_heap(heap);
        return Err(SimpleException::new_msg(ExcType::TypeError, "setattr(): attribute name must be string").into());
    };

    name.drop_with_heap(heap);

    // note: py_set_attr takes ownership of value and drops it on error
    let result = object.py_set_attr(name_id, value, heap, interns);
    object.drop_with_heap(heap);

    result?;

    Ok(Value::None)
}
