class Compiler:
    def __init__(self, code, file):
        self.code = ("".join(list(map(str.strip, code)))).split(";")
        self.file = file
        self.compiled = list()

    def advance(self, line):
        self.line_idx += line
        self.line = self.compiled[self.line_idx]

    def string(self, _str):
        return _str.replace("\s", " ")

    def set_var(self, type):
        self.line = self.line[len(type):]
        self.var_name, self.var_val = self.line.split("=")
        if type == "str":
            self.var_val = self.string(self.var_val)
        self.compiled.extend([
            "set_var",
            self.var_name,
            type,
            self.var_val,
            ""
        ])
    
    def operate_var(self, type, sign):
        self.line = self.line[3:]
        self.dest, self.end = self.line.split("=")
        self.val1, self.val2 = self.end.split(sign)
        if self.val1.startswith("int"):
            self.type1 = "int"
            self.val1 = self.val1[len("int"):]
        elif self.val1.startswith("str"):
            self.type1 = "str"
            self.val1 = self.string(self.val1[len("str"):])
        elif self.val1.startswith("float"):
            self.type1 = "int"
            self.val1 = self.val1[len("float"):]
        if self.val2.startswith("int"):
            self.type2 = "int"
            self.val2 = self.val2[len("int"):]
        elif self.val2.startswith("str"):
            self.type2 = "str"
            self.val2 = self.string(self.val2[len("str"):])
        elif self.val2.startswith("float"):
            self.type2 = "float"
            self.val2 = self.val2[len("float"):] # type length
        self.compiled.extend([
            f"{type}_var",
            self.dest,
            self.type1,
            self.val1,
            self.type2,
            self.val2,
            ""
        ])

    def if_line(self, sign):
        self.sign = sign
        self.val1, self.val2 = self.line.split(sign)
        if self.val1.startswith("int"):
            self.type1 = "int"
            self.val1 = self.val1[len("int"):]
        elif self.val1.startswith("str"):
            self.type1 = "str"
            self.val1 = self.string(self.val1[len("str"):])
        elif self.val1.startswith("float"):
            self.type1 = "int"
            self.val1 = self.val1[len("float"):]
        if self.val2.startswith("int"):
            self.type2 = "int"
            self.val2 = self.val2[len("int"):]
        elif self.val2.startswith("str"):
            self.type2 = "str"
            self.val2 = self.string(self.val2[len("str"):])
        elif self.val2.startswith("float"):
            self.type2 = "int"
            self.val2 = self.val2[len("float"):]
        self.val2 = self.string(self.val2)
        self.val2 = self.string(self.val2)

    def compile(self):
        for self.line in self.code:
            self.line = self.line.replace(" ", "")

            if self.line.startswith("int"):
                self.set_var("int")

            elif self.line.startswith("str"):
                self.set_var("str")
            
            elif self.line.startswith("float"):
                self.set_var("float")
            
            elif self.line.startswith("print"):
                self.line = self.line[5:]
                self.val = self.string(self.line)
                self.compiled.extend([
                    "puts",
                    self.val,
                    ""
                ])
            
            elif self.line.startswith("sum"):
                self.operate_var("sum", "+")
            
            elif self.line.startswith("sub"):
                self.operate_var("sub", "-")
            
            elif self.line.startswith("mul"):
                self.operate_var("mul", "*")

            elif self.line.startswith("div"):
                self.operate_var("div", "/")
            
            elif self.line.startswith("equ"):
                self.line = self.line[3:]
                self.var_name, self.var_val = self.line.split("=")
                self.var_val = self.string(self.var_val)
                self.compiled.extend([
                    "equ_var",
                    self.var_name,
                    self.var_val,
                    ""
                ])

            elif self.line.startswith("input"):
                self.line = self.line[5:]
                self.var_name = self.line
                self.compiled.extend([
                    "cin_var",
                    self.var_name,
                    ""
                ])
            
            elif self.line.startswith("if"):
                self.line = self.line[2:]
                self.line = self.line[self.line.find("(") + 1:self.line.rfind(")")]
                if "=" in self.line:
                    self.if_line("=")
                elif "<" in self.line:
                    self.if_line("<")
                elif ">" in self.line:
                    self.if_line(">")
                elif "!=" in self.line:
                    self.if_line("!=")
                self.compiled.extend([
                    "jumpif",
                    "jump_if_idx",
                    self.sign,
                    self.type1,
                    self.val1,
                    self.type2,
                    self.val2,
                    ""
                ])
            
            elif self.line.startswith("endif"):
                self.compiled.extend([
                    "jump_if_end",
                    ""
                ])
            
            elif self.line.startswith("forever"):
                if self.line.endswith("loop"):
                    self.compiled.extend([
                        "forever_loop_start",
                        ""
                    ])
            
            elif self.line.startswith("endloop"):
                self.compiled.extend([
                    "jump",
                    "jump_idx",
                    ""
                ])
        
        self.compiled.extend(["exit"])

        self.line_idx = -1
        while True:
            try:
                self.advance(1)
            except IndexError:
                break
            if self.line == "jump_if_end":
                self.flag = 0
                self.save_if_jump = self.line_idx
                while True:
                    self.advance(-1)
                    if self.line == "jump_if_idx":
                        if self.flag < 1:
                            self.compiled[self.line_idx] = str(self.save_if_jump + 1)
                            self.line_idx = self.save_if_jump + 1
                            break
                        else:
                            self.flag -= 1
        
        self.line_idx = -1
        while True:
            try:
                self.advance(1)
            except IndexError:
                break
            if self.line == "forever_loop_start":
                self.flag = 0
                self.save_jump = self.line_idx
                while True:
                    self.advance(1)
                    if self.line == "jump_idx":
                        if self.flag < 1:
                            self.compiled[self.line_idx] = str(self.save_jump + 1)
                            self.line_idx = self.save_jump + 1
                            break
                        else:
                            self.flag -= 1

        """
        self.line_idx = -1
        while True:
            try:
                self.advance(1)
            except IndexError:
                break
            print(self.line)
            if self.line == "forever_loop_start":
                print("-")
                self.flag = 0
                self.save_loop_jump = self.line_idx
                while True:
                    self.advance(1)
                    if self.line == "jump_idx":
                        print("#")
                        if self.flag < 1:
                            self.compiled[self.line_idx] = str(self.save_loop_jump + 1)
                            self.line_idx = self.save_loop_jump + 1
                        else:
                            self.flag -= 1
        """

        with open(self.file, "w") as f:
            f.write("\n".join(self.compiled))
