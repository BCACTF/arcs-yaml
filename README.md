# ARCS YAML Parser

This is a simple YAML parser written in Rust to verify the shape of the YAML.
It also can be used as a library for getting a struct of the YAML (as it is used
in the deploy server)

It is subject to change as the deploy and webhook servers evolve.

Install it with `cargo install arcs-ctf_yaml-parser --bin arcs-yaml`

### Note to ARCS developers:

_Because `crates.io` does not support namespaced registries, it is best to
include this crate as a library with
`yaml_verifier = { package = "arcs-ctf_yaml-parser", version = "0.1.0" }`
in your Cargo.toml file._