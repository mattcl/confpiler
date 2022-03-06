**!! This is currently a hobby project, use at your own risk !!**

# Confpiler (cli) #

This cli tool is intended to "compile" an ordered set of configuration files
into a single, flattened representation suitable for exporting to environment
variables.

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


**Currently this tool requires a "default" file when processing a directory.**


### Checking ###

Substituting `check` for `build` will just verify whether or not the
configuration _could_ be made given the options specified.

```sh
$ confpiler check global.yaml myapp --env staging

# or stricter
$ confpiler check global.yaml myapp --env staging --strict
```

For a complete list of sucommands/option (and more detailed help), see the
relevant `--help` section.
