import time

from interpreter import Interpreter
from compiler import Compiler

# Shell
while True:
    inp = input(">> ").split()
    if inp[0] == "run":
        Time = time.time()
        file = inp[1]
        with open(file, "r") as f:
            data = f.readlines()
        language = Interpreter(data)
        language.run()
        Time = time.time() - Time
    if inp[0] == "compile":
        Time = time.time()
        file1 = inp[1]
        file2 = inp[2]
        with open(file1, "r") as f:
            data = f.readlines()
            compiler = Compiler(data, file2)
            compiler.compile()
        Time = time.time() - Time
        
    if inp[0] == "time":
        print(Time)

    if inp[0] == "exit":
        exit()
