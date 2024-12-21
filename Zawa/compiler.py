class Compiler:
    def __init__(self, code, file):
        self.code = ("".join(list(map(str.strip, code)))).split(";")
        self.file = file
        self.compiled = list()

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
                
        self.compiled.extend(["exit"])
        with open(self.file, "w") as f:
            f.write("\n".join(self.compiled))
