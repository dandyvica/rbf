import unittest
from xml.dom import minidom
from datetime import datetime, date, time
from rbf.fieldtype import BaseType
from rbf.fieldtype import FieldType

class TestFieldType(unittest.TestCase):

    def setUp(self):
        self.assertRaises(ValueError, FieldType, "A/N", "STR")
        self.ft = FieldType("A/N", "string")

    def test_0(self):
        self.assertEqual(self.ft.base, str)

    def test_xml_cons(self):
        self.doc = minidom.parseString('<fieldtype name="AN" type="string" pattern="[\w/\*\.,\-]+" format="%-*.*s"/>')
        self.xml_node = self.doc.childNodes[0]
        self.ft = FieldType.from_xml_node(self.xml_node)
        self.assertEqual(self.ft.pattern, "[\w/\*\.,\-]+")
        self.assertEqual(self.ft.format, "%-*.*s")

    def test_conversion(self):
        self.doc = minidom.parseString('<fieldtype name="D" type="date" pattern="[0-9]+" date_format="%Y%m%d"/>')
        self.xml_node = self.doc.childNodes[0]
        self.ft = FieldType.from_xml_node(self.xml_node)
        self.assertEqual(self.ft.date_format, "%Y%m%d")

        self.assertEqual(self.ft.convert("20160226"), datetime(2016, 2, 26))

        self.doc = minidom.parseString('<fieldtype name="T" type="time" pattern="[0-9]+" time_format="%H%M"/>')
        self.xml_node = self.doc.childNodes[0]
        self.ft = FieldType.from_xml_node(self.xml_node)
        self.assertEqual(self.ft.time_format, "%H%M")

        self.assertEqual(self.ft.convert("1210"), datetime(1900,1,1,12,10,0))        

if __name__ == '__main__':
    unittest.main()

