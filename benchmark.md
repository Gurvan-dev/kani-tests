Weird: No problem found for `check_add1` even though the harness was not
correct? It should only be equal up to bound length

## Reference

- check_add1

```
Not supported
```

- check_memset2

```
VERIFICATION:- SUCCESSFUL
Verification Time: 53.712685s
```

## BitStatic Length of 2

- check_add1

```
VERIFICATION:- SUCCESSFUL
Verification Time: 461.13535s
```

- check_memset2

```
VERIFICATION:- SUCCESSFUL
Verification Time: 74.19892s
```

- Tests:

```
running 2 tests
test rv64::test_memset1 ... ok
test rv64::test_add1 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 1.15s
```

## BitStatic Length of 4

- check_add1

```
Killed
```

- check_memset2:

```
VERIFICATION:- SUCCESSFUL
Verification Time: 99.62914s
```

- Tests:

```
running 2 tests
test rv64::test_memset1 ... ok
test rv64::test_add1 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 1.70s
```

## BitStatic Length of 8

- check_add1

```
Killed
```

- check_memset2:

```
VERIFICATION:- SUCCESSFUL
Verification Time: 149.90933s
```

- Tests:

```
running 2 tests
test rv64::test_memset1 ... ok
test rv64::test_add1 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 2.31s
```
