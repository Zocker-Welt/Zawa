<h1 align="center">Zawa</a></h1>

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

<h1 align="center">Documentation</a></h1>
<h3 align="center">Variables</a></h3>
<p align="center">
    Variables are declared when you first use them
</p>

```ruby
int $var = 5; # Sets the variable $var to an integer 5;
str $var = Hello; # Sets the variables $var to a string Hello;

sum $var = int 5 + int 3; # Add 2 values;
sub $var = int 5 - int 3; # Substract 2 values;
mul $var = int 7 * int 4; # Multiply 2 values;
div $var = int 9 / int 4; # Divide 2 values;

equ $var = (5 + 3) / 5; # Smart equation (if you want to use strings put them in "");
```

<h3 align="center">Input / Output</a></h3>

```ruby
print Hello\n; # Writes a value in the terminal;
input $var; # Asks for user input (reads a whole line)
```

<h1 align="center">Thank you!</a></h1>
<p align="center">
    Thanks for reading the whole documentation, more features are coming soon!
</p>
