# Test error cases for del operation

try:
    x = [1, 2, 3]
    del x[10]
    assert False, 'Should have raised IndexError'
except IndexError:
    print('IndexError for out of bounds - OK')

try:
    y = [1, 2, 3]
    del y[-10]
    assert False, 'Should have raised IndexError'
except IndexError:
    print('IndexError for negative out of bounds - OK')

try:
    d = {'a': 1, 'b': 2}
    del d['c']
    assert False, 'Should have raised KeyError'
except KeyError:
    print('KeyError for missing key - OK')

try:
    num = 42
    del num[0]  # pyright: ignore[reportIndexIssue]
    assert False, 'Should have raised TypeError'
except TypeError as e:
    assert 'does not support item deletion' in str(e)

try:
    lst = [1, 2, 3]
    del lst['string']
    assert False, 'Should have raised TypeError'
except TypeError as e:
    assert 'indices' in str(e)
