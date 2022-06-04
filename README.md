# calc

_Do math from the command line_
_Do everything from the command line_

## But why?
I spend 80-90% of my day working in the terminal. Sometimes I need to do math. Rather than getting my phone out or trying to remember how to do math with bash's wacky syntax, why not have a CLI tool that will do simple math stuff for me? And why not write it in Rust?

## Usage
### Doing Math
Passing an equation as an argument to the function will return the answer.

```bash
$ calc 12 + 30
42
```

Quotes are only needed around the equation when using multiplication or division because the `/` and `*` characters will be expanded by your shell.

```bash
$ calc "21 * 2"
42
```

### Converting units
To switch to unit conversion mode, use the `--convert` (`-c`) flag. Conversions are denoted using the any of the following syntax:

- `3.5 ft -> in`
- `42 hp to W`
- `1351.310 lbm - slug` (I'm not a fan of this syntax but at least you won't have to use quotes around this one)


```bash
$ calc -c "3.5 -> in"
42 inch
```

### Unit Aliases
This tool uses the [rink]() library which doesn't recognize `in` as an abbreviation for `inch`. There is some logic to handle this so you can continue to use `in` like a real American. If you find other units that aren't recognized and should be, you can add them to the `aliases` vector in the `get_unit_aliases` function in a little PR.
