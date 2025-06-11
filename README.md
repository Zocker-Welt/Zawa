<h1 align="center">Zawa 0.8.0</a></h1>

<h1 align="center">DO NOT USE THIS LANGUAGE. EVERYTHING IS BROKEN AND IT WAS MADE BY A COKE ADDICT</a></h1>

<p align="center">
    <img width="400" src="https://github.com/Zocker-Welt/Zawa/blob/main/wiki/Zawa%20logo%20v3.png" alt="logo">
</p>

<p align="center">
  <b>
    The Zawa programming language
  </b>
</p>

<p align="center">
  You need to have <a href="https://www.python.org/downloads/">Python</a> installed to compile/run your programm
</p>

<h1 align="center">About</a></h1>

<p align="center">
    Zawa is a virtual compiled programming language which was made in Python
</p>

<p align="center">
    It compiles your .zawa source file into a .zvm file, which is then executed by the interpreter
</p>

<h1 align="center">Usage</a></h1>

<p align="left">
    Open the shell (make sure you are in the correct dir)
</p>

```bash
python shell.py
```

<p align="left">
    Compile your .zawa into a .zvm file (These files won't be created so you will need to manually create them)
</p>

```bash
compile main.zawa main.zvm
```

<p align="left">
    Run your .zvm file
</p>

```bash
run main.zvm
```

<p align="left">
    Find out the time of compile/run
</p>

```bash
time
```

<p align="left">
    Close the shell
</p>

```bash
exit
```

<h1 align="center">Documentation</a></h1>

<p align="center">
  <b>
    \s = space character
  </b>
</p>
<p align="center">
  <b>
    \n = new line character
  </b>
</p>
<p align="center">
  <b>
    \t = tab character
  </b>
</p>

<h3 align="center">Variables</a></h3>
<p align="center">
    Variables are declared when you first use them
</p>

```ruby
int $var = 5; # Sets the variable $var to an integer 5;
float $var = 3.4; # Sets the variable $var to a a floating point 3.4;
str $var = Hello,\sworld!\n; # Sets the variables $var to a string Hello;

sum $var = int 5 + int 3; # Add 2 values;
sub $var = int 5 - int 3; # Substract 2 values;
mul $var = int 7 * int 4; # Multiply 2 values;
div $var = int 9 / int 4; # Divide 2 values;

equ $var = (5 + 3) / 5; # Smart equation (if you want to use strings put them in "");
```

<h3 align="center">Input / Output</a></h3>

```ruby
print Hello,\sworld!\n; # Writes a value in the terminal;
input $var; # Asks for user input (reads a whole line)
```

<h3 align="center">Logic</a></h3>

```ruby
if (int 4 == int 4) do; # You can write ==, !=, <, >;
    print Math \s ain't \s mathing\n; # Indentation is not important;
endif;
```

<h3 align="center">Loops</a></h3>

```ruby
forever loop;
    print nani\n;
endloop;
```

<p align="left">
    breake a loop
</p>

```ruby
breakloop;
```

<p align="left">
    Create a for loop
</p>

```ruby
# Counts from 0 to 4;
int $i = 0;
forever loop;
    print $i;
    print \n;
    sum $i = int $i + int 1;
    if (int $i == int 5) do;
        breakloop;
    endif;
endloop;
```

<h3 align="center">Using_name</a></h3>

<p align="left">
    😅 You can't use ( for do because ( and ) are reserved for ifs (in more updates for lists to)
</p>

```ruby
using_name { = loop;
using_name } = endloop;

using_name [ = do;
using_name ] = endif;

# Makes so you can write beautiful for loops and ifs;

int $i = 1;
forever {;
    print $i;
    print \n;
    sum $i = int $i + int 1;
    if (int $i == int 1001) [;
        breakloop;
    ];
};
```

<p align="left">
    It may also be used for 1 line macros
</p>

```ruby
using_name hello = print Hello, \s world!\n;

hello;
```

<h1 align="center">.zvm</a></h1>

<p align="center">
  <b>
    Here I'll explain how to write the .zvm file
  </b>
</p>

<p align="center">
    The language structure is the same, but you'll need to write more lines
</p>

<h3 align="center">Variables</a></h3>

<p align="left">
    set_var
</p>

```
set_var
$var
int
4

set_var
$var
str
Hello
```

<p align="left">
    operation_var
</p>

```
sum_var
$var
int
5
int
3

sub_var
$var
int
5
int
3

mul_var
$var
int
7
int
4

div_var
$var
int
9
int
4

```

<p align="left">
    equ_var
</p>

```
equ_var
$var
(5+3)/5
```

<p align="left">
    random_var
</p>

```
random_var
$var
-5
5
```

<h3 align="center">Input / Output</a></h3>

```
puts
Hello, world\n

cin_var
$var
```

<h3 align="center">Jumps</a></h3>

<p align="left">
    jump to a line (while True loops)
</p>

```
jump
line_idx
```

<p align="left">
    jumpif (if statements)
</p>
<p align="left">
    you can use < = > ! for expressions
</p>
<p align="left">
    if the statement is incorrect we jump to a line else we do nothing  
</p>

```
jumpif
line_idx
>
int
5
int
3
```

<h3 align="center"><a href="https://github.com/Zocker-Welt/Zawa/blob/main/wiki/game.zvm">Example</a></h3>
<p align="center">
  <b>
    A simple guessing game
  </b>
</p>

<h1 align="center">Links</a></h1>
<p align="center">
  <a href="https://www.youtube.com/watch?v=4GDkzBva0OE">Tutorial episode 1</a>
</p>

<h1 align="center">Credits</a></h1>
<p align="center">
    Thanks to <a href="https://scratch.mit.edu/users/redspacecat/">redspacecat</a> for explaining me how to make a virtual machine compiled language
</p>

<p align="center">
    Thanks for reading the whole documentation, more features are coming soon!
</p>

<p align="center">
    The first 100% working version of Zawa will be 1.0 (still no error handling xD)
</p>
