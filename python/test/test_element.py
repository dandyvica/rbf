import unittest
from rbf.element import Element

class TestElement(unittest.TestCase):

    def test(self):
        self.e1 = Element("ELEMENT1", "Alpha element 1", 10)
        self.e2 = Element("ELEMENT1", "Alpha element 1", 10)

        self.assertRaises(ValueError, Element, "", "Alpha element 1", 10)
        self.assertRaises(ValueError, Element, "ELEMENT1", "Alpha element 1", -1)

        self.assertEqual(self.e1.name, "ELEMENT1")
        self.assertEqual(self.e1.description, "Alpha element 1")
        self.assertEqual(self.e1.length, 10)

        self.assertEqual(self.e1, self.e2)

        self.e1.name = "ELEMENT2"
        self.e1.description = "Alpha element 2"
        self.e1.length = 20

        self.assertEqual(self.e1.name, "ELEMENT2")
        self.assertEqual(self.e1.description, "Alpha element 2")
        self.assertEqual(self.e1.length, 20)

        self.assertEqual(self.e1.__repr__(), "name=<ELEMENT2> description=<Alpha element 2> length=<20>")

if __name__ == '__main__':
    unittest.main()

