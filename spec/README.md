# dotenv

## 参考实现

- [dotenv for JS][dotenv-js]
- [dotenv-expand for JS][dotenv-expand-js]
- [dotenv for Python][dotenv-python]
- [posix shell][posix-shell]

[dotenv-js]: https://www.npmjs.com/package/dotenv
[dotenv-expand-js]: https://www.npmjs.com/package/dotenv-expand
[dotenv-python]: https://pypi.org/project/python-dotenv/
[posix-shell]: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html

## dotenv for JS

```js
const LINE = /(?:^|^)\s*(?:export\s+)?([\w.-]+)(?:\s*=\s*?|:\s+?)(\s*'(?:\\'|[^'])*'|\s*"(?:\\"|[^"])*"|\s*`(?:\\`|[^`])*`|[^#\r\n]+)?\s*(?:#.*)?(?:$|$)/mg
```

## dotenv for Python

```python
def make_regex(string: str, extra_flags: int = 0) -> Pattern[str]:
    return re.compile(string, re.UNICODE | extra_flags)


_newline = make_regex(r"(\r\n|\n|\r)")
_multiline_whitespace = make_regex(r"\s*", extra_flags=re.MULTILINE)
_whitespace = make_regex(r"[^\S\r\n]*")
_export = make_regex(r"(?:export[^\S\r\n]+)?")
_single_quoted_key = make_regex(r"'([^']+)'")
_unquoted_key = make_regex(r"([^=\#\s]+)")
_equal_sign = make_regex(r"(=[^\S\r\n]*)")
_single_quoted_value = make_regex(r"'((?:\\'|[^'])*)'")
_double_quoted_value = make_regex(r'"((?:\\"|[^"])*)"')
_unquoted_value = make_regex(r"([^\r\n]*)")
_comment = make_regex(r"(?:[^\S\r\n]*#[^\r\n]*)?")
_end_of_line = make_regex(r"[^\S\r\n]*(?:\r\n|\n|\r|$)")
_rest_of_line = make_regex(r"[^\r\n]*(?:\r|\n|\r\n)?")
_double_quote_escapes = make_regex(r"\\[\\'\"abfnrtv]")
_single_quote_escapes = make_regex(r"\\[\\']")
```

## dotenv for Ruby

```ruby
LINE = /
      (?:^|\A)              # beginning of line
      \s*                   # leading whitespace
      (?:export\s+)?        # optional export
      ([\w.]+)              # key
      (?:\s*=\s*?|:\s+?)    # separator
      (                     # optional value begin
        \s*'(?:\\'|[^'])*'  #   single quoted value
        |                   #   or
        \s*"(?:\\"|[^"])*"  #   double quoted value
        |                   #   or
        [^\#\r\n]+          #   unquoted value
      )?                    # value end
      \s*                   # trailing whitespace
      (?:\#.*)?             # optional comment
      (?:$|\z)              # end of line
    /x
```
