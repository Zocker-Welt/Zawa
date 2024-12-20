import random
from eval import Evaluater

class Interpreter:
    def __init__(self, code):
        self.code = list()
        for i in code:
            if i.endswith("\n"):
                i = i[:-1]
            elif i.startswith("\t") or i.startswith(" "):
                while i.startswith("\t") or i.startswith(" "):
                    i = i[1:]
            self.code.append(i)
        self.code.insert(0, "")
        self.variables = dict()
    
    def advance(self):
        self.line_idx += 1
        self.line = self.code[self.line_idx]
    
    def get_val(self, text, type):
        if text in self.variables:
            self.val = self.variables[text]
        else:
            self.val = text
        if type == "int":
            self.val = int(self.val)
        elif type == "str":
            self.val = str(self.val)
        return self.val
    
    def equation(self, sign, val1, val2):
        if sign == "=":
            if val1 == val2:
                return True
            return False
        elif sign == ">":
            if val1 > val2:
                return True
            return False
        elif sign == "<":
            if val1 < val2:
                return True
            return False
        elif sign == "!":
            if val1 != val2:
                return True
            return False

    def run(self):
        self.line_idx = -1
        self.line = str()
        while self.line != "exit":
            self.advance()
            if self.line == "set_var":
                self.advance()
                self.name = self.line
                self.advance()
                self.type = self.line
                self.advance()
                self.val_text = self.line
                self.val = self.get_val(self.val_text, self.type)
                self.variables[self.name] = self.val
            
            elif self.line == "cin_var":
                self.ioinput = input()
                self.advance()
                self.name = self.line
                self.variables[self.name] = self.ioinput # str

            elif self.line == "sum_var":
                self.advance()
                self.dest_var = self.line
                self.advance()
                self.type1 = self.line
                self.advance()
                self.val_text1 = self.line
                self.val1 = self.get_val(self.val_text1, self.type1)
                self.advance()
                self.type2 = self.line
                self.advance()
                self.val_text2 = self.line
                self.val2 = self.get_val(self.val_text2, self.type2)
                self.variables[self.dest_var] = self.val1 + self.val2

            elif self.line == "sub_var":
                self.advance()
                self.dest_var = self.line
                self.advance()
                self.type1 = self.line
                self.advance()
                self.val_text1 = self.line
                self.val1 = self.get_val(self.val_text1, self.type1)
                self.advance()
                self.type2 = self.line
                self.advance()
                self.val_text2 = self.line
                self.val2 = self.get_val(self.val_text2, self.type2)
                self.variables[self.dest_var] = self.val1 - self.val2

            elif self.line == "mul_var":
                self.advance()
                self.dest_var = self.line
                self.advance()
                self.type1 = self.line
                self.advance()
                self.val_text1 = self.line
                self.val1 = self.get_val(self.val_text1, self.type1)
                self.advance()
                self.type2 = self.line
                self.advance()
                self.val_text2 = self.line
                self.val2 = self.get_val(self.val_text2, self.type2)
                self.variables[self.dest_var] = self.val1 * self.val2

            elif self.line == "div_var":
                self.advance()
                self.dest_var = self.line
                self.advance()
                self.type1 = self.line
                self.advance()
                self.val_text1 = self.line
                self.val1 = self.get_val(self.val_text1, self.type1)
                self.advance()
                self.type2 = self.line
                self.advance()
                self.val_text2 = self.line
                self.val2 = self.get_val(self.val_text2, self.type2)
                self.variables[self.dest_var] = self.val1 / self.val2
            
            elif self.line == "equ_var":
                self.advance()
                self.dest_var = self.line
                self.advance()
                self.expression = self.line
                _eval = Evaluater(self.variables)
                self.variables[self.dest_var] = _eval.evaluate(self.expression)
            
            elif self.line == "random_var":
                self.advance()
                self.dest_var = self.line
                self.advance()
                self.val_text1 = self.line
                self.val1 = self.get_val(self.val_text1, "int")
                self.advance()
                self.val_text2 = self.line
                self.val2 = self.get_val(self.val_text2, "int")
                self.variables[self.dest_var] = random.randint(int(self.val1), int(self.val2))

            elif self.line == "puts":
                self.advance()
                self.val_text = self.line
                #print("sdf",self.val_text)
                if self.val_text in self.variables:
                    self.val = self.variables[self.val_text]
                else:
                    self.val = self.val_text
                self.val = self.val.encode().decode('unicode_escape') if isinstance(self.val, str) else self.val
                print(self.val, end="")
            
            elif self.line == "jump":
                self.advance()
                self.val_text = self.line
                self.val = self.get_val(self.val_text, "int")
                self.line_idx = self.val
            
            elif self.line == "jumpif":
                self.advance()
                self.if_line = self.line
                self.advance()
                self.sign = self.line

                self.advance()
                self.type1 = self.line
                self.advance()
                self.val_text1 = self.line
                self.val1 = self.get_val(self.val_text1, self.type1)

                self.advance()
                self.type2 = self.line
                self.advance()
                self.val_text2 = self.line
                self.val2 = self.get_val(self.val_text2, self.type2)

                self.bool_equation = self.equation(self.sign, self.val1, self.val2)
                
                if not(self.bool_equation):
                    self.line_idx = int(self.if_line)