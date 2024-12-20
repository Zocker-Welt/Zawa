class Evaluater:
    def __init__(self, variables):
        self.variables = variables

    def evaluate(self, expression):
        self.tokens = expression
        for var_name, var_val in self.variables.items():
            self.tokens = self.tokens.replace(var_name, str(var_val))  # Update self.tokens
        self.ans = eval(self.tokens)
        return self.ans