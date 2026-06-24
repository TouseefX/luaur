# Test Attribution

The integration tests in this directory are **adapted from
[mlua](https://github.com/mlua-rs/mlua)**, which is distributed under the MIT
License.

They have been ported to the `luaur-rt` API (the mlua-style, pure-Rust ergonomic
surface over `luaur`/Luau) as a *behavioral compatibility proof*: where a test
passes unchanged against `luaur-rt`, it demonstrates that `luaur-rt` matches
mlua's observable behavior. Tests that exercise mlua features `luaur-rt` has
intentionally deferred were dropped, and tests that hit a documented `luaur-rt`
deviation were adapted with a `// DEVIATION:` note.

Phase 1 added behavioral coverage for threads/coroutines (`tests/mlua_thread.rs`),
`RegistryKey`, `UserDataFields`, typed `AnyUserData` read-back
(`borrow`/`borrow_mut`/`take`/`is`/`type_id`), the `MetaMethod` enum, and
`Function::info`/`environment`. Still deferred (later phases): async, scopes,
serde, buffers, vectors, `Send`/`Sync`, the proc-macro `chunk!`, and
`#[derive(UserData)]`.

## Adapted files

| luaur-rt test file        | adapted from mlua |
|---------------------------|-------------------|
| `tests/mlua_function.rs`   | `tests/function.rs`   |
| `tests/mlua_table.rs`      | `tests/table.rs`      |
| `tests/mlua_value.rs`      | `tests/value.rs`      |
| `tests/mlua_conversion.rs` | `tests/conversion.rs` |
| `tests/mlua_userdata.rs`   | `tests/userdata.rs`   |
| `tests/mlua_thread.rs`     | `tests/thread.rs`     |
| `tests/mlua_string.rs`     | `tests/string.rs`     |
| `tests/mlua_error.rs`      | `tests/error.rs`      |
| `tests/mlua_multi.rs`      | `tests/multi.rs`      |
| `tests/mlua_chunk.rs`      | `tests/chunk.rs`      |
| `tests/mlua_luau.rs`       | `tests/luau.rs` (Luau-relevant subset) |

## mlua MIT License

```
MIT License

Copyright (c) 2019-2021 A. Orlenko
Copyright (c) 2017 rlua

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
