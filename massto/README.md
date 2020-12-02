<p align="center">
  <h1>rust-ldap</h1>
  <br>

  A Pure-Rust LDAP Library using Tokio & Futures.
  <br>

  <a href="https://crates.io/crates/ldap">
      <img src="https://img.shields.io/crates/d/ldap.svg" alt="rust-ldap on crates.io">
  </a>
  <a href="https://docs.rs/ldap">
      <img src="https://docs.rs/ldap/badge.svg" alt="docs: release versions documentation">
  </a>
</p>

Feel free to join #rust-ldap on Mozilla IRC for questions & general chat.


### RFC compliance

- [x] Bind (4.2)
- [ ] Unbind (4.3)
- [ ] Search (4.5)
- [ ] Modify (4.6)
- [ ] Add (4.7)
- [ ] Delete (4.8)
- [ ] Modify DN (4.9)
- [ ] Compare (4.10)
- [ ] Abandon (4.11)
- [ ] Extended Operation (4.12)
- [ ] TLS / STARTTLS (4.14 / 5)

### rfc4515 (Search Filter String Representation)

The search filter crate [has moved](https://github.com/dequbed/rfc4515).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
