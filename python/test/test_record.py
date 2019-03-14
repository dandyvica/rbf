import sys
import unittest

from rbf.fieldtype import FieldType
from rbf.field import Field
from rbf.record import Record

class TestRecord(unittest.TestCase):

    def setUp(self):
        self.ft = FieldType("A/N", "string")
        self.rec = Record("RECORD1", "Description of record 1")
        self.rec.append(Field("FIELD1", "Description of field 1", self.ft, 10))
        self.rec.append(Field("FIELD2", "Description of field 2", self.ft, 5))
        self.rec.append(Field("FIELD2", "Description of field 2", self.ft, 5))
        self.rec.append(Field("FIELD3", "Description of field 3", self.ft, 10))

        self.line = "A"*10 + "B"*5 + "C"*5 + "D"*10

    def test_0(self):
        self.assertRaises(ValueError, Record, "", "Description of record 1")

        with self.assertRaises(ValueError):
            f = self.rec["FOO"]

    def test_1(self):
        self.assertEqual(self.rec.name, "RECORD1")
        self.assertEqual(self.rec.description, "Description of record 1")
        self.assertEqual(self.rec.length, 30)
        self.assertEqual(len(self.rec), 30)
        self.assertEqual(self.rec.count(), 4)

    def test_2(self):
        self.f1 = self.rec[0]
        self.f2 = self.rec[1]
        self.f3 = self.rec[2]
        self.f4 = self.rec[3]

        self.assertEqual(self.f1.name, "FIELD1")
        self.assertEqual(self.f2.name, "FIELD2")
        self.assertEqual(self.f3.name, "FIELD2")
        self.assertEqual(self.f4.name, "FIELD3")

        self.assertEqual(self.f1.index, 0)
        self.assertEqual(self.f2.index, 1)
        self.assertEqual(self.f3.index, 2)
        self.assertEqual(self.f4.index, 3)

        self.assertEqual(self.f1.offset, 0)
        self.assertEqual(self.f2.offset, 10)
        self.assertEqual(self.f3.offset, 15)
        self.assertEqual(self.f4.offset, 20)

    def test_3(self):
        f = self.rec["FIELD1"]
        self.assertEqual(len(f), 1)

        f = self.rec[2:4]
        self.assertEqual(f[1].name, "FIELD3")

    def test_4(self):
        self.rec.value = self.line
        self.assertEqual(self.rec.value, self.line)

        self.assertEqual(self.rec.FIELD1, "A"*10)
        self.assertEqual(self.rec.FIELD3, "D"*10)

    def test_5(self):
        self.assertEqual(self.rec.array_of('name'), ["FIELD1", "FIELD2", "FIELD2", "FIELD3"])

    def test_6(self):
        self.f1 = self.rec[0]
        self.f2 = self.rec[1]
        self.f3 = self.rec[2]
        self.f4 = self.rec[3]

        it = iter(self.rec)

        self.assertEqual(next(it), self.f1)
        self.assertEqual(next(it), self.f2)
        self.assertEqual(next(it), self.f3)
        self.assertEqual(next(it), self.f4)

    def test_7(self):
        self.rec.value = self.line
        self.assertEqual(self.rec.array_of('name'), ['FIELD1', 'FIELD2', 'FIELD2', 'FIELD3'])

    def test_8(self):
        self.rec.value = self.line
        self.assertEqual(self.rec.as_dict(), {'FIELD1': 'AAAAAAAAAA', 'FIELD3': 'DDDDDDDDDD', 'FIELD2': ['BBBBB', 'CCCCC']})

    def test_9(self):
        self.rec.delete(['FIELD2', 'FIELD1'])
        self.assertEqual('FIELD1' in self.rec, False)
        self.assertEqual('FIELD2' in self.rec, False)
        self.assertEqual('FIELD3' in self.rec, True)

    def test_10(self):
        self.rec.keep(['FIELD2'])
        self.assertEqual('FIELD1' in self.rec, False)
        self.assertEqual('FIELD2' in self.rec, True)
        self.assertEqual('FIELD3' in self.rec, False)

    def test_11(self):
        self.rec = Record("RECORD1", "Description of record 1")

        # read data from external file
        for line in open("./record.data"):
            a = [s.strip() for s in line.split(';')]
            self.rec.append(Field(a[0], a[1], self.ft, int(a[2])))

        self.assertEqual(len(self.rec), 400)
        self.assertEqual(self.rec.count(), 94)

        self.assertEqual(len(self.rec["COUN"]), 4)
        self.assertEqual(self.rec[28].name, "COUN")
        
        self.assertEqual("COUN" in self.rec, True)
        self.assertEqual("XXXX" not in self.rec, True)

        self.assertEqual("COUN" in self.rec.array_of('name'), True)

        l = self.rec.array_of('length')
        self.assertEqual(sum(l), 400)

        self.rec.delete(["COUN","RCID"])
        self.assertEqual("COUN" not in self.rec, True)
        with self.assertRaises(ValueError):
            f = self.rec["COUN"]

        self.rec.keep(["CABI","STPO","ORAC"])
        self.assertEqual("CABI" in self.rec, True)
        self.assertEqual(len(self.rec.array_of('name')), 4*3)

if __name__ == '__main__':
    unittest.main()

