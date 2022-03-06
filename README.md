**This is currently a hobby project.** While it currently works as advertised
(at least on linux), patches may be slow.

# Confpiler #

A library and cli tool for "compiling" an ordered set of configuration files
into a single, flattened representation suitable for exporting to environment
variables.

[The Twelve-Factor App](https://12factor.net) concept has been around for
a while now, and, while the section on configuration makes good recommendations
about how a conforming app should load its configuration, it doesn't specify
_how_ those variables are set in the first place.

The intent of this tool is to allow for defining configuration values in a more
human-manageable format, then enabling the export of those values as
environment variables.

Specific READMEs:

  * [library README](confpiler/README.md)
  * [cli README](confpiler_cli/README.md)

## CLI ##

### A simple example: ###

```yaml
# given a file, config.yaml
foo:
  bar: 10
  baz: false

hoof: https://some.url
```

```sh
$ confpiler build config.yaml
FOO__BAR="10"
FOO__BAZ="false"
HOOF="https://some.url"
```

### A more complicated example ###

Given some files like the following:

```
somedir/
  global.yaml
  myapp/
    default.yaml
    development.yaml
    production.yaml
    staging.yaml
```

We can compile to a single representation of the "production" configuration
with

```sh
$ cd somedir
$ confpiler build global.yaml myapp --env production --json
```

Which would yield a dictionary in JSON form representing merging `global.yaml`,
`myapp/default.yaml` and `myapp/production.yaml`.


**Currently the cli tool requires a "default" file when processing
a directory.**


## Library ##

See the [library README](confpiler/README.md)


## Supported formats ##

The following formats are currently supported:

  * JOSN
  * TOML
  * YAML
  * INI
