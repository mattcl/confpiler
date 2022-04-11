# Confpiler (crate) #

This crate provides a mechanism for "compiling" an ordered set of configuration
files into a single, flattened representation suitable for exporting to
environment variables.

Transforming

```text
## default.yaml
foo:
    bar: 10
    baz: false
hoof: doof

## production.yaml
foo:
    baz: true
```

into something like

```text
"FOO__BAR": "10"
"FOO__BAZ": "false"
"HOOF": "doof"
```

via

```rust no_run
use confpiler::FlatConfig;

let (conf, warnings) = FlatConfig::builder()
    .add_config("foo/default")
    .add_config("foo/production")
    .build()
    .expect("invalid config");
```

All values are converted to strings, with simple arrays being collapsed to
delimited strings (with the default separator being `,`).

This does not support arrays containing more complex values like other arrays
and maps.

### The following formats are currently supported: ###

  * JOSN
  * TOML
  * YAML
  * INI
