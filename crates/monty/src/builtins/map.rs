//! Implementation of the map() builtin function.

use crate::{
    args::{ArgValues, KwargsValues},
    exception_private::{ExcType, RunResult, SimpleException},
    heap::{Heap, HeapData},
    intern::Interns,
    io::NoPrint,
    resource::ResourceTracker,
    types::{List, MontyIter, PyTrait},
    value::Value,
};

/// Implementation of the map() builtin function.
///
/// Applies a function to every item of one or more iterables and returns a list of results.
/// With multiple iterables, stops when the shortest iterable is exhausted.
///
/// Note: In Python this returns an iterator, but we return a list for simplicity.
/// Note: The `strict=` parameter is not yet supported.
///
/// Examples:
/// ```python
/// map(abs, [-1, 0, 1, 2])           # [1, 0, 1, 2]
/// map(pow, [2, 3], [3, 2])          # [8, 9]
/// map(str, [1, 2, 3])               # ['1', '2', '3']
/// ```
pub fn builtin_map(heap: &mut Heap<impl ResourceTracker>, args: ArgValues, interns: &Interns) -> RunResult<Value> {
    let (mut positional, kwargs) = args.into_parts();

    if !kwargs.is_empty() {
        for (k, v) in kwargs {
            k.drop_with_heap(heap);
            v.drop_with_heap(heap);
        }
        for v in positional {
            v.drop_with_heap(heap);
        }
        return Err(
            SimpleException::new_msg(ExcType::TypeError, "map() does not support keyword arguments yet").into(),
        );
    }

    // must have at least function + one iterable
    if positional.len() < 2 {
        for v in positional {
            v.drop_with_heap(heap);
        }
        return Err(SimpleException::new_msg(ExcType::TypeError, "map() must have at least two arguments.").into());
    }

    let function = positional.next().unwrap();

    // note we don't support user-defined functions yet
    let builtin = match function {
        Value::Builtin(b) => b,
        not_supported => {
            let func_type = not_supported.py_type(heap);
            not_supported.drop_with_heap(heap);
            for v in positional {
                v.drop_with_heap(heap);
            }
            return Err(
                SimpleException::new_msg(ExcType::TypeError, format!("'{func_type}' object is not callable")).into(),
            );
        }
    };

    function.drop_with_heap(heap);

    let mut iterators: Vec<MontyIter> = Vec::with_capacity(positional.len() - 1);
    for iterable in positional {
        match MontyIter::new(iterable, heap, interns) {
            Ok(iter) => iterators.push(iter),
            Err(e) => {
                for iter in iterators {
                    iter.drop_with_heap(heap);
                }
                return Err(e);
            }
        }
    }

    let mut out = Vec::new();

    // map function over iterables until the shortest iter is exhausted
    'outer: loop {
        let mut items = Vec::with_capacity(iterators.len());

        for iter in &mut iterators {
            if let Some(item) = iter.for_next(heap, interns)? {
                items.push(item);
            } else {
                for v in items {
                    v.drop_with_heap(heap);
                }

                break 'outer;
            }
        }

        let args = match items.len() {
            0 => ArgValues::Empty,
            1 => ArgValues::One(items.into_iter().next().unwrap()),
            2 => {
                let mut iter = items.into_iter();
                ArgValues::Two(iter.next().unwrap(), iter.next().unwrap())
            }
            _ => ArgValues::ArgsKargs {
                args: items,
                kwargs: KwargsValues::Empty,
            },
        };

        let result_value = builtin.call(heap, args, interns, &mut NoPrint)?;
        out.push(result_value);
    }

    for i in iterators {
        i.drop_with_heap(heap);
    }

    let heap_id = heap.allocate(HeapData::List(List::new(out)))?;
    Ok(Value::Ref(heap_id))
}
