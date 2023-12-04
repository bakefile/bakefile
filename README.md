# `Bakefile` 🍞

---

[![Continuous Integration](https://github.com/bakefile/bakefile/actions/workflows/main.yml/badge.svg)](https://github.com/bakefile/bakefile/actions/workflows/main.yml)

---

# ⚠️
<!-- # ⚠️ ACHTUNG! ⚠️ -->
<!-- # ⚠️ ATENCIÓN! ⚠️ -->
<!-- # ⚠️ ATENÇÃO! ⚠️ -->
<!-- # ⚠️ ATTENTION! ⚠️ -->
<!-- # ⚠️ ATTENZIONE! ⚠️ -->
<!-- # ⚠️ WARNING! ⚠️ -->

# 🚧👷 STILL WORK-IN-PROGRESS 👷🚧

---

```shell
cargo install bakefile
```

`bake` is kind of like `make` in that it runs subprocesses but its
domain makes references to baking and its comments takes currency
symbols.

## Features:

- Each `Bakefile` is a "recipe"
- Each "recipe" declares "instructions"
- Each "instruction" can contain either "steps" or "dependencies" or both
- Uses 4 spaces for steps
- Tabs are not supported
- Uncommonly allows currency symbols as comments

## USAGE

```shell
bake
```
