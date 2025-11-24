use monty::{Executor, Exit};

macro_rules! id_tests {
    ($($name:ident: $code:literal, $expected:expr;)*) => {
        $(
            paste::item! {
                #[test]
                fn [< id_ $name >]() {
                    let mut ex = Executor::new($code, "test.py", &[]).unwrap();
                    let result = ex.run(vec![]);
                    let output = match result {
                        Ok(Exit::Return(obj)) => format!("{}: {}", obj.type_str(), obj.repr()),
                        otherwise => panic!("Unexpected exit: {:?}", otherwise),
                    };
                    let expected = $expected.trim_matches('\n');
                    assert_eq!(output, expected);
                }
            }
        )*
    }
}

id_tests! {
    // Singleton consistency - each singleton should return the same ID every time
    singleton_none_consistent: "id(None) == id(None)", "bool: True";
    singleton_true_consistent: "id(True) == id(True)", "bool: True";
    singleton_false_consistent: "id(False) == id(False)", "bool: True";
    singleton_ellipsis_consistent: "id(...) == id(...)", "bool: True";

    // Singletons should have different IDs from each other
    singleton_none_vs_true: "id(None) == id(True)", "bool: False";
    singleton_none_vs_false: "id(None) == id(False)", "bool: False";
    singleton_true_vs_false: "id(True) == id(False)", "bool: False";
    singleton_ellipsis_vs_none: "id(...) == id(None)", "bool: False";
    singleton_ellipsis_vs_true: "id(...) == id(True)", "bool: False";

    // Different integer values should have different IDs
    int_different_values: "id(10) == id(20)", "bool: False";
    int_zero_vs_one: "id(0) == id(1)", "bool: False";

    // Integer variable assignment - copying creates separate identity
    // Note: unlike Python's small int interning, Monty creates separate boxed values
    int_assignment_separate_identity: "
x = 100
y = x
id(x) == id(y)
", "bool: False";

    // Integer reassignment creates new value with different identity
    int_reassignment_different: "
x = 100
y = 200
id(x) == id(y)
", "bool: False";

    // String identity - different string objects have different IDs
    str_different_instances: "id('hello') == id('hello')", "bool: False";
    str_assignment_same_id: "
s = 'test'
t = s
id(s) == id(t)
", "bool: True";

    // List identity - different list objects have different IDs even if equal
    list_different_empty: "id([]) == id([])", "bool: False";
    list_different_same_content: "id([1, 2, 3]) == id([1, 2, 3])", "bool: False";
    list_var_consistent: "
lst = [1, 2, 3]
id(lst) == id(lst)
", "bool: True";
    list_assignment_same_id: "
lst = [1, 2, 3]
ref = lst
id(lst) == id(ref)
", "bool: True";

    // Tuple identity
    tuple_different_empty: "id(()) == id(())", "bool: False";
    tuple_different_same_content: "id((1, 2)) == id((1, 2))", "bool: False";
    tuple_var_consistent: "
t = (1, 2, 3)
id(t) == id(t)
", "bool: True";
    tuple_assignment_same_id: "
t = (1, 2, 3)
ref = t
id(t) == id(ref)
", "bool: True";

    // Float identity - different float values have different IDs
    float_different_values: "id(1.5) == id(2.5)", "bool: False";

    // Mixed type comparisons - different types should have different IDs
    int_vs_str: "id(42) == id('42')", "bool: False";
    int_vs_list: "id(1) == id([1])", "bool: False";
    int_vs_tuple: "id(1) == id((1,))", "bool: False";
    true_vs_int_one: "id(True) == id(1)", "bool: False";
    false_vs_int_zero: "id(False) == id(0)", "bool: False";

    // List modification doesn't change ID
    list_append_same_id: "
lst = [1]
old_id = id(lst)
lst.append(2)
old_id == id(lst)
", "bool: True";

    // Nested structures
    nested_list_different: "id([[1]]) == id([[1]])", "bool: False";
    nested_list_var_consistent: "
nested = [[1, 2], [3, 4]]
id(nested) == id(nested)
", "bool: True";

    // Multiple variables pointing to same object share ID
    multiple_refs_same_id: "
original = [1, 2, 3]
ref1 = original
ref2 = original
ref3 = ref1
(id(original) == id(ref1), id(ref1) == id(ref2), id(ref2) == id(ref3))
", "tuple: (True, True, True)";

    // Bytes identity
    bytes_var_consistent: "
b = b'hello'
id(b) == id(b)
", "bool: True";
    bytes_different_instances: "id(b'test') == id(b'test')", "bool: False";
    bytes_assignment_same_id: "
b = b'data'
ref = b
id(b) == id(ref)
", "bool: True";
}
