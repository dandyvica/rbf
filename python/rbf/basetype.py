"""
Define a basic types used by fields
"""

import re
from datetime import datetime, date, time

class BaseType(object):
    """ 
    List all possible base field types:
        * string
        * decimal
        * integer
        * date
        * time

        The conv key in the **valid_types** dict has 2 argmuments because for some conversion functions,
        2 arguments are needed (e.g.: date/time conversion)
    """
    valid_types = {
            "string": {"base": str, "conv": lambda x,y: x, "init": "", "pattern": re.compile("[\w/\*\.,\-]+"), "format":"%-*.*s"},
            "integer": {"base": int, "conv": lambda x,y: int(x), "init": 0, "pattern": re.compile("[0-9]+"), "format":"%0*.*d"},
            "decimal": {"base": float, "conv": lambda x,y: float(x), "init": 0.0, "pattern": re.compile("[0-9]+"), "format":"%0*.2g"},
            "date": {"base": datetime, "conv": lambda x,y: datetime.strptime(x,y), "init": "0", "pattern": re.compile("[0-9]+"), "format":""},
            "time": {"base": datetime, "conv": lambda x,y: datetime.strptime(x,y), "init": "0", "pattern": re.compile("[0-9]+"), "format":""}
    }

    def __init__(self, type_as_string):
        """
        create a new base type based solely on its string name. Other attributes come along.

        :param str type_as_string: string name of the base type

        """
        if type_as_string not in BaseType.valid_types: 
            raise ValueError("%s in not a valid type !!" % type_as_string)

        self.type_as_string = type_as_string
        self.basic_type     = BaseType.valid_types[type_as_string]

        # dynamically create attributes from keys. We can reset those attributes to other values if needed
        for k,v in self.basic_type.items():
            setattr(self, k, v)


    def __repr__(self) -> str:
        return "{0}".format(self.type_as_string)

    def convert(self, value, additional_arg=""):
        """ 
        convert a value according to the underlying field type conversion function

        :param str value: value to convert from

        """
        converted_value = value

        try:
            converted_value = self.conv(value, additional_arg)
        except ValueError:
            print("Unable to convert value <%s> for type %s:%s" % (value, self.type_as_string, self.basic_type))
        
        return converted_value

    def match(self, value):
        """
        Test if a value matches the base type pattern

        :param str value: value to test

        """
        return self.pattern.match(value)



