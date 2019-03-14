import unittest
from datetime import datetime, date, time
from xml.dom import minidom
from rbf.fieldtype import FieldType
from rbf.field import Field

class TestField(unittest.TestCase):

    def setUp(self):
        self.ft1 = FieldType("AN", "string")
        self.f1 = Field("FIELD1", "Alpha field", self.ft1, 10)

        self.ft2 = FieldType("I", "integer")
        self.f2 = Field("FIELD2", "Integer field", self.ft2, 10)

        self.ft3 = FieldType("N", "decimal")
        self.f3 = Field("FIELD3", "Decimal field", self.ft3, 10)

        self.ft4 = FieldType("D", "date")
        self.f4 = Field("FIELD4", "Date field", self.ft4, 8)

        self.ft5 = FieldType("T", "time")
        self.f5 = Field("FIELD5", "Time field", self.ft5, 4)

    def test_bad_cons(self):
        self.assertRaises(ValueError, Field, "", "Alpha field 1", self.ft1, 10)
        self.assertRaises(ValueError, Field, "FIELD1", "Alpha field", self.ft1, -1)

    """
    def test_other_cons(self):
        self.doc = minidom.parseString('<field name="FIELD" description="Field desc" length="10" type="AN"/>')
        self.xml_node = self.doc.childNodes[0]
        self.other_f = Field.from_xml_node(self.xml_node)
        self.assertEqual(self.other_f.name, "FIELD")
        self.assertEqual(self.other_f.description, "Field desc")
        self.assertEqual(self.other_f.ftype.name, "AN")
        self.assertEqual(self.other_f.length, 10)
    """

    def test_equality(self):
        self.assertEqual(self.f1, Field("FIELD1", "Alpha field", self.ft1, 10))

    def test_properties(self):
        self.assertEqual(self.f1.name, "FIELD1")
        self.assertEqual(self.f1.description, "Alpha field")
        self.assertEqual(self.f1.ftype.name, "AN")
        self.assertEqual(self.f1.length, 10)

    def test_set_value(self):
        self.f1.value = "    XXX"
        self.assertEqual(self.f1.value, "XXX")
        self.assertEqual(self.f1.raw_value, "    XXX")

    def test_init(self):
        self.f1.initialize()
        self.assertEqual(self.f1.raw_value, " "*self.f1.length)

        self.f2.initialize()
        self.assertEqual(self.f2.raw_value, "0"*self.f2.length)

        self.f3.initialize()
        self.assertEqual(self.f3.raw_value, "0"*self.f3.length)

    def test_reset(self):
        setattr(self.ft1, "format", "%*.*s")
        self.f1.reset("AAA")
        self.assertEqual(self.f1.raw_value, " "*7+"AAA")

        setattr(self.ft1, "format", "%-*.*s")
        self.f1.reset("AAA")
        self.assertEqual(self.f1.raw_value, "AAA"+" "*7)

        setattr(self.ft2, "format", "%0*d")
        self.f2.reset(314)
        self.assertEqual(self.f2.raw_value, "0000000314")

        setattr(self.ft2, "format", "%*d")
        self.f2.reset(314)
        self.assertEqual(self.f2.raw_value, "       314")

        setattr(self.ft3, "format", "%0*.2f")
        self.f3.reset(3.14)
        self.assertEqual(self.f3.raw_value, "0000003.14")

        setattr(self.ft3, "format", "%*.2f")
        self.f3.reset(3.14)
        self.assertEqual(self.f3.raw_value, "      3.14")

    def test_convert(self):
        self.f4.value = "20000101"

    

if __name__ == '__main__':
    unittest.main()

