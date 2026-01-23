# Test del operation on lists and dicts

x = [1, 2, 3, 4, 5]
del x[2]
assert x == [1, 2, 4, 5], f'Expected [1, 2, 4, 5], got {x}'

y = [10, 20, 30, 40]
del y[-1]
assert y == [10, 20, 30], f'Expected [10, 20, 30], got {y}'

z = ['a', 'b', 'c']
del z[0]
assert z == ['b', 'c'], f"Expected ['b', 'c'], got {z}"

d = {'a': 1, 'b': 2, 'c': 3}
del d['b']
assert d == {'a': 1, 'c': 3}, f"Expected {{'a': 1, 'c': 3}}, got {d}"
assert len(d) == 2, f'Expected length 2, got {len(d)}'

d2 = {1: 'one', 2: 'two', 3: 'three'}
del d2[2]
assert d2 == {1: 'one', 3: 'three'}, f"Expected {{1: 'one', 3: 'three'}}, got {d2}"
