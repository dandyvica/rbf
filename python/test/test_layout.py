import sys
import unittest

from rbf.layout import Layout

class TestReader(unittest.TestCase):

    def setUp(self):
        self.assertRaises(ValueError, Layout, "foo.xml")
        self.layout = Layout("world_data.xml")

        # test meta data
        self.assertEqual(self.layout.version, "1.0")
        self.assertEqual(self.layout.description, "Continents, countries, cities")
        self.assertEqual(self.layout.schema, "world_data")
        self.assertEqual(self.layout.ignoreLine, "^#")
        self.assertEqual(self.layout.skipField, "ID")
        self.assertEqual(self.layout.mapper, "type:1 map:0..4")

        self.assertEqual(len(self.layout.records()), 2)

        for rec in self.layout:
            self.assertEqual(rec.name in ["COUN","CONT"], True)

    def test1(self):
        self.layout.delete(['COUN'])
        self.assertEqual(len(self.layout.records()), 1)

    def test2(self):
        self.layout = Layout("world_data.xml")
        self.layout.simplify(["CONT:NAME,AREA", "COUN:POPULATION"])
        self.assertEqual(self.layout["CONT"].array_of('name'), ["NAME","AREA"])



if __name__ == '__main__':
    unittest.main()

