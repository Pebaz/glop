
class Fixed:
    def __init__(self, value, precision):
        assert precision <= 32, 'Can only store 32 bits of precision'

        self.value = value << precision
        self.precision = precision

        # All least significant bits set, all most significant bits clear
        self.fraction_divisor = (1 << self.precision)
        self.fraction_mask = self.fraction_divisor - 1

    def __add__(self, other):
        # TODO(pbz): Account for differing precision or throw error
        if isinstance(other, Fixed):
            self.value += other.value
        else:
            self.value += other.value << self.precision
        return self

    def __sub__(self, other):
        if isinstance(other, Fixed):
            self.value -= other.value
        else:
            self.value -= other.value << self.precision
        return self

    def __mul__(self, other):
        self.value *= other.value
        return self

    def __div__(self, other):
        self.value /= other.value
        return self

    def __str__(self):
        fraction = float(
            self.value & self.fraction_mask
        ) / self.fraction_divisor
        return f'{self.value >> self.precision}{str(fraction)[1:]}'

    def __repr__(self):
        return str(self)

    def __format__(self, *args):
        return str(self)
