from interpreter import Interpreter
from compiler import Compiler

def prep(data):
    prep_data = list()
    for i in data:
        if i.endswith("\n"):
            i = i[:-2]
        elif i.startswith("\t") or i.startswith(" "):
            while i.startswith("\t") or i.startswith(" "):
                i = i[1:]
    return prep_data

# Shell
while True:
    inp = input(">> ").split()
    if inp[0] == "run":
        file = inp[1]
        with open(file, "r") as f:
            data = f.readlines()
        language = Interpreter(data)
        language.run()
    if inp[0] == "compile":
        file1 = inp[1]
        file2 = inp[2]
        with open(file1, "r") as f:
            data = f.readlines()
            compiler = Compiler(data, file2)
            compiler.compile()

    if inp[0] == "exit":
        exit()