class Evaluater:
    def __init__(self, expression, variables):
        self.variables = variables
        self.tokens = expression

    def evaluate(self):
        for var_name, var_val in self.variables.items():
            self.tokens = self.tokens.replace(var_name, str(var_val))  # Update self.tokens
        self.ans = eval(self.tokens)
        return self.ans
