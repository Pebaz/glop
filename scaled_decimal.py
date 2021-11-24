class Decimal:
    "https://docs.oracle.com/javadb/10.6.2.1/ref/rrefsqlj36146.html"

    def __init__(self, value):
        self.value = value

    def __add__(self, other):
        integer1, decimal1 = self.value.split('.')
        integer2, decimal2 = other.value.split('.')
        new_integer = int(integer1) + int(integer2)
        difference = abs(len(decimal1) - len(decimal2))

        if len(decimal1) > len(decimal2):
            decimal2 += '0' * difference

        elif len(decimal1) < len(decimal2):
            decimal1 += '0' * difference

        new_decimal = ''
        carry = 0
        for i in reversed(range(len(decimal1))):
            l = int(decimal1[i])
            r = int(decimal2[i])

            new = l + r + carry
            carry = int(new >= 10)

            if carry:
                new %= 10

            new_decimal = f'{new}{new_decimal}'

        if carry:
            new_integer += carry

        self.value = f'{new_integer}.{new_decimal}'
        return self

    def __mul__(self, other):
        return self

    def __div__(self, other):
        return self

    def __str__(self):
        return self.value

    def __repr__(self):
        return self.value
