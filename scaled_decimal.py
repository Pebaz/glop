class Decimal:
    "https://docs.oracle.com/javadb/10.6.2.1/ref/rrefsqlj36146.html"

    def __init__(self, integer, decimal):
        self.integer = integer
        self.decimal = decimal
        self.precision = self.num_digits(integer) + self.num_digits(decimal)
        self.scale = self.num_digits(decimal)

    def num_digits(self, number):
        count = 0
        while number > 0:
            count += 1
            number //= 10
        return count

    def __add__(self, other):
        new_precision = 2 * (self.precision - self.scale) + self.scale
        new_scale = max(self.scale, other.scale)  # And all others

        new_integer = self.integer + other.integer
        new_decimal = self.decimal + other.decimal

        self.integer = new_integer
        self.decimal = new_decimal

        return self


    def __mul__(self, other):
        new_precision = self.precision + other.precision
        new_scale = self.scale + other.scale


    def __div__(self, other):
        new_scale = self.precision - self.scale + other.precision + max(
            self.scale + other.precision - other.scale + 1,
            4
        )
        new_scale = 31 - self.precision + self.scale - other.scale

    def __str__(self):
        decimal = ''.join(reversed(str(self.decimal)))
        return f'{self.integer}.{decimal}'

    def __repr__(self):
        return str(self)
