""" useful function to modify overpunched fields into regular ASCII fields """
from rbf.field import Field
from rbf.record import Record

# static symbols for mapping overpunched char to regular chars
dish_pos = str.maketrans("{ABCDEFGHI}", "01234567890")
dish_neg = str.maketrans("JKLMNOPQR", "123456789")


def ispunch(s):
    """
    tests if a string is an overpunched string, i.e. including chars in {ABCDEFGHIJKLMNOPQR}
    returns 1 is s contains positive overpunch char, -1 if negative, 0 if not overpunch

    """
    for letter in "{ABCDEFGHI}":
        if letter in s: return 1
    for letter in "JKLMNOPQR":
        if letter in s: return -1
    return 0


def overpunch(record):
    """ modify all fields of a record if field is numeric and an overpunched value is found """
    for f in record:
        if f.field_type == "N" or f.field_type == "I":
            punchtest = ispunch(f.value)
            if punchtest == 1:
                f.value = f.value.translate(dish_pos)
            elif punchtest == -1:
                f.value = f.value.translate(dish_neg)

