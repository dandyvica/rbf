import unittest
from datetime import datetime, date, time
from rbf.basetype import BaseType

class TestBaseType(unittest.TestCase):

    def setUp(self):
        self.assertRaises(ValueError, BaseType, "foo")

    def test_0(self):
        self.ft = BaseType("string")
        self.assertEqual(self.ft.basic_type["base"], str)

    def test_1(self):
        self.ft = BaseType("string")
        self.assertEqual(self.ft.convert("3.14"), "3.14")

        self.ft = BaseType("decimal")
        self.assertEqual(self.ft.convert("3.14"), 3.14)

        self.ft = BaseType("integer")
        self.assertEqual(self.ft.convert("314"), 314)

        self.ft = BaseType("date")
        self.ft.convert("XXXX0226", "%Y%m%d")
        self.assertEqual(self.ft.convert("20160226", "%Y%m%d"), datetime(2016, 2, 26))

        self.ft = BaseType("time")
        self.assertEqual(self.ft.convert("121013", "%H%M%S"), datetime(1900,1,1,12,10,13))
    
    def test_2(self):
        self.ft = BaseType("string")
        self.assertEqual(bool(self.ft.match("AAA")), True)


if __name__ == '__main__':
    unittest.main()

