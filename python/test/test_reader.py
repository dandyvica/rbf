import sys
import unittest

from rbf.layout import Layout
from rbf.reader import Reader

class TestReader(unittest.TestCase):

    def setUp(self):
        self.layout = Layout("world_data.xml")
        self.reader = Reader("world_data.txt", self.layout, lambda x: x[0:4])
 
    def test_bad_conv(self):
        self.assertRaises(ValueError, Reader, "foo.txt", None, lambda x: x[0:4])

    def test_loop1(self):
        import itertools
        top5 = list(itertools.islice(self.reader, 5))
        rec = top5[4]
        self.assertEqual(";".join(rec.array_of('value')), "COUN;China Tibet;2620000;Lhasa")

    def test_loop2(self):
        self.l = list(self.reader)
        #self.assertEqual(";".join(self.l[4].array_of('value')), "COUN;China Tibet;2620000;Lhasa")


if __name__ == '__main__':
    unittest.main()

