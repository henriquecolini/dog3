# `dog3`

This is the repository for `dog3`, an experimental, fun, and somewhat esoteric programming language.
<br>
`dog3` is a mix of Rust and Bash, and is designed to be as easy to use as it is hard to do anything useful with.

The following `dog3` program prints "Hello world!" to the console:

```
pln Hello world!
```
```
> Hello world!
```

However, this one does not:

```
{ pln Hello world! }
```
```
>
```

So, what happened there? It's simple: the output of the function `pln` got **captured** by the scope delimited by the brackets, and this scope was never used for anything. In dog3, the very act of _printing_ values is, in fact, the only way of using them. All operations you do, from math to processing data, are made entirely through printing - which we will simply call **"outputting"** from now on.

To make stuff work, function outputting is fully scoped - that is, whenever a function outputs something, it is appended to its scope, as if the block was a text file.

A block scope is a value by itself, and can be used in exactly the same places as a string can. The following code does indeed work:

```
put { pln Hello world! }
```
```
> Hello world!
```

The function `put` simply outputs all of its inputs. It's the most basic and most common function you'll ever use. The function `pln` does the same, but it also appends a newline to the end of the output.

Just like in Bash, parameters are delimited by spaces (therefore, in our first example, "Hello" and "world!" were, in fact, two separate arguments). If you want to use spaces in a parameter, you can use quotes or escape them:

```
pln "Double     quotes!";
pln 'Single     quotes!';
pln Escape\ \ \ those\ \ \ spaces!;
```
```
> Double     quotes!
> Single     quotes!
> Escape   those   spaces!
```

Some characters are special and can't appear in an unquoted parameter: `;` `,` `{` `}` `(` `)` `=` `$` `%` `'` `"` `[` `]` `` ` `` `&`. To use them as text, put them inside quotes (`pln "Hello, world!"`).

In `dog3`, variables work very similar to Bash as well:

```
x = 5;
pln "I'm $x$x years old!";
pln "That snake is $x' long!";
pln '20$x5 = 100$'
```
```
> I'm 55 years old!
> That snake is 5' long!
> 20$x5 = 100$
```

Single quotes don't expand variables. Also note that a variable name runs until the next whitespace or special character, so `$x!` refers to a variable called `x!`, not `x` followed by `!`.

And remember: blocks are values, just like strings:

```
x = {
    put Foo;
    put Bar
};
put $x$x
```
```
> FooBarFooBar
```

Do notice the usage of semicolons. You must always include a semicolon after a function call or non-block statement, except for the last statement in a block.

You can nest blocks as much as you want:

```
x = {
    y = {
        z = {
            put foo
        };
        put bar $z bar
    };
    put baz $y baz
};
pln $x
```
```
> baz bar foo bar baz
```

You may begin to wonder: what data types are available on `dog3`, since everything is just printed?

**There's only one data type in `dog3`: string.**

Well, not exactly. Indeed, you won't be able to use numbers, arrays, or anything useful like that - it really is all strings. That said, the strings themselves carry alongside them an invisible value: the **status code**.

The status code is a number that is used to determine the truthiness of a string. A string with a status code of 0 is truthy, and any other value is falsy. You may be familiar with this concept if you've ever used Bash. You can use the function `status` to get the status code of a string:

```
status "Hello, world!"
```
```
> 0
```

The command `status` can also be used to set the status code of a string:

```
status "I hold a secret!" 42
```
```
> I hold a secret!
```

Knowing this, we can talk about conditionals:

```
if { eq 1 1 } {
    pln 1 is equal to 1
}
else {
    pln 1 is not equal to 1
}
```
```
>
```

Wait, that outputted nothing! Of course: the `if` block is a scope, and so, the output of `pln` was captured by it. To bubble it out, we need to use `put`:

```
put if { eq 1 1 } {
    pln 1 is equal to 1
}
else {
    pln 1 is not equal to 1
}
```
```
> 1 is equal to 1
```

Same logic applies to `while` loops:

```
x = 0;
put while { lt $x 10 } {
    put $x;
    x = { add $x 1 }
}
```
```
> 0123456789
```

We can also use `for` loops, to iterate over... strings split by whitespace, of course.

```
put for x in "dog3    is my  favorite  language" {
    pln - $x
}
```
```
> - dog3
> - is
> - my
> - favorite
> - language
```

Additionally, you can split a string by any separator you want with `split`:

```
put for x in "ideal,for,csv" split "," {
    pln - $x
}
```
```
> - ideal
> - for
> - csv
```

So far, we've only used the built-in functions of `dog3`. You can also define your own functions using the `fn` keyword:

```
fn add_one (x) {
    put { add $x 1 }
}

fn say_hello () {
    pln Hello world!
}

// Yes, overloading is supported!
fn say_hello (name) {
    pln Hello $name
}

pln { add_one 5 };
say_hello;
say_hello dog3
```
```
> 6
> Hello world!
> Hello dog3
```

Functions take specific amounts of arguments. If you want an arbitrary amount of arguments, you can prefix a `%` to the end of the last argument name. This will join all remaining arguments into a single string, separated by spaces:

```
fn hug (l,r,%args) {
    put for x in $args {
        put $l$x$r
    }
}

hug < > a b c d e f;
```
```
> <a><b><c><d><e><f>
```

Two keywords change what a block outputs. `return` stops the enclosing function and makes the given value its output, discarding everything the function output so far. Outside a function, `return` ends the whole program. `clear` throws away everything the current block has output so far, replacing it with the given value (or nothing):

```
fn find (target, %list) {
    put for x in $list {
        if { eq $x $target } {
            return "found $x"
        };
        put "skipped $x\n"
    };
    put "not found"
}

pln { find 3 1 2 3 4 };
pln { put a; put b; clear c; put d }
```
```
> found 3
> cd
```

Here are all the functions you can currently use. The list is small as the language is very young, and will increase over time.

### Module `std`

| Function | Description | Status |
| -------- | ----------- | ------ |
| put %args | Outputs `args` | status `args` |
| pln %args | Outputs `args` followed by a newline | status `args` |
| print %args | Writes `args` to standard output | truthy |
| println %args | Writes `args` to standard output followed by a newline | truthy |
| status arg | Returns the status code of `arg` | status `arg` |
| status arg st | Outputs `arg` | `st` |
| panic message | Stops the program with `message` as the error | — |
| src name | Outputs the source code of every overload of function `name` (built-ins appear as stubs) | truthy |
| eval code | Parses and runs `code` as a separate program that can use the current functions, then outputs its output | status of the program's output, or falsy with the error message if it fails |

### Module `iter`

| Function | Description | Status |
| -------- | ----------- | ------ |
| range max | Outputs "0 1 2 ... `max`-1" | truthy if `max` is an integer, falsy otherwise |
| range min max | Outputs "`min` `min`+1 ... `max`-1" | truthy if all arguments are integers, falsy otherwise |
| range min max step | Outputs every `step`th number from `min` up to (excluding) `max` | truthy if all arguments are integers and `step` >= 0, falsy otherwise |
| range min max step sep | Same as above, separated by `sep` instead of spaces | truthy if all arguments are integers and `step` >= 0, falsy otherwise |
| len arr | Outputs length of `arr`, split by whitespaces | truthy |
| len arr sep | Outputs length of `arr`, split by `sep` | truthy |
| first arr n | Outputs first `n` elements of `arr`, split by whitespaces | truthy if n <= len, falsy otherwise |
| first arr n sep | Outputs first `n` elements of `arr`, split by `sep` | truthy if n <= len, falsy otherwise |
| last arr n | Outputs last `n` elements of `arr`, split by whitespaces | truthy if n <= len, falsy otherwise |
| last arr n sep | Outputs last `n` elements of `arr`, split by `sep` | truthy if n <= len, falsy otherwise |
| append left right | Outputs the combined split by whitespaces of `left` and `right` chained | truthy |
| append left right sep | Outputs the combined split by `sep` of `left` and `right` chained | truthy |

### Module `logic`

| Function | Description | Status |
| -------- | ----------- | ------ |
| true | Outputs "" | truthy
| false | Outputs "" | falsy
| eq a b | Outputs "" | truthy if a = b numerically, falsy otherwise
| neq a b | Outputs "" | truthy if a != b numerically, falsy otherwise
| lt a b | Outputs "" | truthy if a < b numerically, falsy otherwise
| gt a b | Outputs "" | truthy if a > b numerically, falsy otherwise
| leq a b | Outputs "" | truthy if a <= b numerically, falsy otherwise
| geq a b | Outputs "" | truthy if a >= b numerically, falsy otherwise
| like a b | Outputs "" | truthy if a == b textually, falsy otherwise
| and a b | Outputs "" | truthy if status a && status b, falsy otherwise
| or a b | Outputs "" | truthy if status a \|\| status b, falsy otherwise
| not a | Outputs "" | truthy if status a != 0, falsy otherwise

### Module `math`

| Function | Description | Status |
| -------- | ----------- | ------ |
| add first %others | Outputs decimal float `first` + `others` | truthy if all arguments are numbers, falsy otherwise
| sub first %others | Outputs decimal float `first` - `others` | truthy if all arguments are numbers, falsy otherwise
| mul first %others | Outputs decimal float `first` \* `others` | truthy if all arguments are numbers, falsy otherwise
| div first %others | Outputs decimal float `first` / `others` (divides sequentially) | truthy if all arguments are numbers, falsy otherwise
| max first %others | Outputs decimal float max(`first`, `others`) | truthy if all arguments are numbers, falsy otherwise
| min first %others | Outputs decimal float min(`first`, `others`) | truthy if all arguments are numbers, falsy otherwise
| floor x | Outputs decimal float floor(`x`) | truthy if all arguments are numbers, falsy otherwise
| ceil x | Outputs decimal float ceil(`x`) | truthy if all arguments are numbers, falsy otherwise
| random max | Outputs integer random number between 0 (inclusive) and `max` (exclusive) | truthy if all arguments are numbers, falsy otherwise
| random min max | Outputs integer random number between `min` (inclusive) and `max` (exclusive) | truthy if all arguments are numbers, falsy otherwise

### Module `str`

| Function | Description | Status |
| -------- | ----------- | ------ |
| upper %args | Outputs `args` in uppercase | status `args`
| lower %args | Outputs `args` in lowercase | status `args`
| replace target from to | Outputs `target` with all occurrences of `from` replaced by `to` | status `target`
| search target pattern | Outputs the lines of `target` that match the regular expression `pattern` | truthy if `pattern` is a valid regex, falsy otherwise
| is_alpha %args | Outputs "" | truthy if every character of `args` is alphabetic, falsy otherwise
| is_alphanumeric %args | Outputs "" | truthy if every character of `args` is alphanumeric, falsy otherwise

### Module `net`

| Function | Description | Status |
| -------- | ----------- | ------ |
| get url | Outputs the response of an HTTP GET request on `url` | the status code of the response, unless it was 200, in which case truthy
| get url timeout | Outputs the response of an HTTP GET request on `url`, waiting for at most `timeout` seconds | the status code of the response, unless it was 200, in which case truthy
| post url | Outputs the response of an HTTP POST request on `url` | the status code of the response, unless it was 200, in which case truthy
| post url body | Outputs the response of an HTTP POST request on `url`, with body `body` | the status code of the response, unless it was 200, in which case truthy
| post url body timeout | Outputs the response of an HTTP POST request on `url`, with body `body`, waiting for at most `timeout` seconds | the status code of the response, unless it was 200, in which case truthy

### Module `json`

| Function | Description | Status |
| -------- | ----------- | ------ |
| gron input | Outputs a flattened (grepabble) version of the JSON `input` | truthy if `input` is a valid JSON
| jstr string | Outputs `string` as a JSON string (quoted and escaped) | truthy
| jnum number | Outputs `number` as a JSON number, or `null` | truthy if `number` is a valid number, falsy otherwise
| jarr %items | Outputs a JSON array of `items`; items that are valid JSON are kept as is, others become strings | truthy
| jobj %keys_values | Outputs a JSON object from alternating keys and values (values as in `jarr`) | truthy
| jstr_parse json_string | Outputs the text inside a JSON string | truthy if `json_string` is a JSON string, falsy otherwise
| jpush object %items | Outputs the JSON array `object` with `items` appended | truthy
| jlen object | Outputs the length of a JSON array or object, or `null` | truthy if `object` is an array or object, falsy otherwise
| jget object key | Outputs the element at `key` of a JSON object, or at index `key` of an array, or `null` | truthy if found, falsy otherwise
| jkeys object | Outputs the keys of a JSON object as a JSON array, or `null` | truthy if `object` is an object, falsy otherwise