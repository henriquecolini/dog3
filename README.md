# `dog3`

This is the repository for `dog3`, an experimental, fun, and somewhat esoteric programming language.
<br>
`dog3` is a mix of Rust and Bash, and is designed to be as easy to use as it is hard to do anything useful with.

The following `dog3` program prints "Hello, world!" to the console:

```
pln Hello, world!
```
```
> Hello, world!
```

However, this one does not:

```
{ pln Hello, world! }
```
```
>
```

So, what happened there? It's simple: the output of the function `pln` got **captured** by the scope delimited by the brackets, and this scope was never used for anything. In dog3, the very act of _printing_ values is, in fact, the only way of using them. All operations you do, from math to processing data, are made entirely through printing - which we will simply call **"outputting"** from now on.

To make stuff work, function outputting is fully scoped - that is, whenever a function outputs something, it is appended to its scope, as if the block was a text file.

A block scope is a value by itself, and can be used in exactly the same places as a string can. The following code does indeed work:

```
put { pln Hello, world! }
```
```
> Hello, world!
```

The function `put` simply outputs all of its inputs. It's the most basic and most common function you'll ever use. The function `pln` does the same, but it also appends a newline to the end of the output.

Just like in Bash, parameters are delimited by spaces (therefore, in our first example, "Hello," and "world!" were, in fact, two separate arguments). If you want to use spaces in a parameter, you can use quotes or escape them:

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

In `dog3`, variables work very similar to Bash as well:

```
x = 5;
pln I'm $x$x years old!;
pln "That snake is $x' long!";
pln '20$x5 = 100$'
```
```
> I'm 55 years old!
> That snake is 5' long!
> 20$x5 = 100$
```

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
    pln $x!
}
```
```
> dog3!
> is!
> my!
> favorite!
> language!
```

Additionally, you can split a string by any separator you want:

```
put for x in "ideal,for,csv" "," {
    pln $x...
}
```
```
> ideal...
> for...
> csv...
```

So far, we've only used the built-in functions of `dog3`. You can also define your own functions using the `fn` keyword:

```
fn add_one (x) {
    put { add $x 1 }
}

fn say_hello () {
    pln Hello, world!
}

// Yes, overloading is supported!
fn say_hello (name) {
    pln Hello, $name!
}

add_one 5;
say_hello;
say_hello dog3"
```
```
> 6
> Hello, world!
> Hello, dog3!
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

Here are all the functions you can currently use. The list is small as the language is very young, and will increase over time.

### Module `std`

| Function | Description | Status |
| -------- | ----------- | ------ |
| put %in | Outputs `in` | status `in` |
| pln %in | Outputs `in` followed by a newline | status `in` |
| print %in | Writes `in` to standard output | truthy |
| println %in | Writes `in` to standard output followed by a newline | truthy |
| status in | Returns the status code of `in` | status `in` |
| status in st | Outputs `in` | `st` |

### Module `iter`

| Function | Description | Status |
| -------- | ----------- | ------ |
| range n | Outputs "0 1 2 3 ... `$n`" | truthy |
| range n sep | Outputs "0`$sep`1`$sep`2`$sep`3`$sep`...`$sep`n" | truthy |
| len arr | Outputs length of `arr`, split by whitespaces | truthy |
| len arr sep | Outputs length of `arr`, split by `sep` | truthy |
| first arr n | Outputs first `n` elements of `arr`, split by whitespaces | truthy if len <= n, falsy otherwise |
| first arr n sep | Outputs first `n` elements of `arr`, split by `sep` | truthy if len <= n, falsy otherwise |
| last arr n | Outputs last `n` elements of `arr`, split by whitespaces | truthy if len <= n, falsy otherwise |
| last arr n sep | Outputs last `n` elements of `arr`, split by `sep` | truthy if len <= n, falsy otherwise |
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
| or a b | Outputs "" | truthy if status a && status b, falsy otherwise
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
| ceil x | Outputs decimal float ceil(`first`) | truthy if all arguments are numbers, falsy otherwise