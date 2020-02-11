# Overview

Provides a trait for easier expression (and consumption) if features, toggles,
checkboxes, settings, options, or any other so-called _bivalent_ pair.

Conditions typically come in pairs (e.g, `{Deny, Allow}`, `{No, Yes}`, etc.)

A derive macro is available in the `prelude` module. If the macro is not
desired to be in scope, simply use the crate instead.

## Custom Derive

Currently the custom derive macro is applicable to enums with two fields only.
The first field will be treated as the false state, while the second field
will be treated as the true state.

## Roadmap

Currently planned:

 - [ ] `#[true]` and `#[false]` attributes to permit changing the order, as
 well as having multiple names for true or false.
 - [ ] Automatic `FromStr` generation for parsing.

