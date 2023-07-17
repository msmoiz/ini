# INI

This is a simple library for working with INI files. For more information on
usage and supported syntax, see the [crate documentation](/src/lib.rs).

```ini
use ini::Ini;

let ini = Ini::from_str("
    [greeting]
    early=morning
    late=night
").unwrap();

assert_eq!(ini["greeting"]["early"], "morning");
assert_eq!(ini["greeting"]["late"], "night");
```
