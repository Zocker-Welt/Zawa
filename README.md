> [!IMPORTANT]  
> This is a WIP project

> [!NOTE]  
> It's very slow


<h1 align="center">
    Zawa 0.12.0
</h1>

<h1 align="center">
  <img src="zawa_logo.svg" alt="Zawa Logo" style="height: 320px;">
</h1>

<h1 align="center">
    Examples
</h1>

<h3 align="left">
    Fibonacci
</h3>

```js
let a = 0;
let temp;

for (let b = 1; a < 100; b = temp + b) {
    echo a;
    temp = a;
    a = b;
}
```

<h1 align="center">
    Setup
</h1>

<p align="left">
    1. Download zip
</p>

<p align="left">
    2. Extract all
</p>

<p align="left">
    3. Go to the directory
</p>

<p align="left">
    4. Build it
</p>

```ps
cargo b --release
```

<p align="left">
    5. Add to path
</p>

```ps
# Windows
[Environment]::SetEnvironmentVariable("PATH", "$($env:PATH);
Path_to_the_directory_where_zawa_executable_is_located", "User ")
```

```bash
# Linux
vim ~/.bashrc
```
Add line
```bash
export PATH="$PATH:Path_to_the_directory_where_zawa_executable_is_located"
```
exit and apply changes
```bash
source ~/.bashrc
```

<h1 align="center">
    Usage
</h1>

<p align="left">
    Open the shell
</p>

```bash
zawa
```

<p align="left">
    Run code from file
</p>

```bash
zawa [file]
```

<h1 align="center">
    Documentation
</h1>

<h3 align="center">
    Quick tutorial
</h1>

```js
// This is a comment

// Write to io
print(expression)
println(expression)
echo expression; // Outdated (may be removed soon)

// Variable declaration
let name = value;
let name; // the variable's value is null

// Variable assignment
new_var = value;
echo name = value // Variable assignment is an expression (not a statement). It returns the new value

// Or
println("stringval" or false); // Writes stringval
println(false or "stringval"); // Writes stringval


// And
println("stringval" and true); // Writes true
println(true and "stringval"); // Writes stringval


// If and else
if (condition) {
    print("This is an if");
} else {
    // Optional
    println("This is an else");
}

// While loop
while (condition) {
    println("This is a while loop");
}

// For loop
for (initializer; condition; incrementer) {
    println("This is a for loop");
}

// Exit a block (code in {} is a block)
if (condition) {
    println("This will ve written to io");
    break;
    println("This won't be written to io");
}

// Create a function
fn sum(a, b) {
    println(a + b);
    a + b; // The last expression is the return value (If you dont write it the program crashes)
}

// Call a function
sum(1, 2);
let result = sum(1, 2);
```

<h3 align="center">
    Standart Library
</h3>

```js
time() // returns the current time in seconds since the unix time epoch
print(arg) // writes to io
println(arg) // writes a line to io
```

<h3 align="center">
    Data types
</h3>

```js
name:           Number
description:    Floating point number
example:        31.4
```

```js
name:           String
description:    Text value
example:        "Hello, world!"
```

```js
name:           bollean
description:    bollean value
examples:        true, false
```

```js
name:           null
description:    null value
example:        null
```

<h3 align="center">
    Keywords
</h3>

```js
and
or

true
false

if
else
class
self

fn
return

for
while

null

echo

super

let
```

<h3 align="center">
    Operators
</h3>

```js
+ - * / ( )
```
