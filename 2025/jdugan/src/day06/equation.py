class Equation:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, operator, terms):
        self.operator = operator
        self.terms    = terms


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def add_term(self, term):
        self.terms.append(term)

    def calculate(self):
        if self.operator == "+":
            return sum(self.terms)
        else:
            result = 1
            for term in self.terms:
                result *= term
            return result