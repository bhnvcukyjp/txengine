# txengine

The omission of the repo's context, as well as it being a one-off, is intentional.

## Author's assumptions
- The program returns errors if the structure of provided file is invalid (ATMs sometime display errors).
- (Wasn't specified) Disputes to accounts with inssuficient funds are ignored.
- Dispute system only applies to deposits (that's why withdrawals are omitted in the AccountManager).

## Security
- Added clippy flags to prohibit use of unwraps, expects and panics.
- The program does not expose internal state when returning errors.
- Code was analyzed using [cargo audit](https://github.com/rustsec/rustsec?tab=readme-ov-file)

## Maintainability
- Logic split into easily digestible chunks.
- Created macro to avoid repetition of the same code block in integration tests.
- [Cargo machete](https://github.com/bnjbvr/cargo-machete) was used to ensure there are no unused dependencies.
- Added clippy's pedantic flag to get the most detailed feedback.

## Efficiency
- Deserialization is done without allocations and using [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html).
- AccountManager is storing only necessary data (i.e. saving only deposits).

## Correctness
- Provided a suite of integration tests, to see test cases run
```sh
cargo test
```
- Transaction type was created with custom deserializer. That ensures validity of records in the input file (i.e. only deposits and withdrawals have amounts).
