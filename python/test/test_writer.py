import unittest

from rbf.fieldtype import FieldType
from rbf.field import Field
from rbf.record import Record
from rbf.writer.writer import writer, WriterStyle

class TestElement(unittest.TestCase):

    def test(self):
        self.writer = writer("", WriterStyle.text)

        self.ft = FieldType("A/N", "string")
        self.rec = Record("RECORD1", "Description of record 1")
        self.rec.append(Field("LONG_FIELD1", "Description of field 1", self.ft, 10))
        self.rec.append(Field("LONG_FIELD2", "Description of field 2", self.ft, 5))
        self.rec.append(Field("LONG_FIELD2", "Description of field 2", self.ft, 5))
        self.rec.append(Field("LONG_FIELD3", "Description of field 3", self.ft, 10))

        self.line = "A"*10 + "B"*5 + "C"*5 + "D"*10
        self.rec.value = self.line

        self.writer.to_tag(self.rec)

        self.assertEqual('RECORD1:LONG_FIELD1="AAAAAAAAAA" LONG_FIELD2="BBBBB" LONG_FIELD2="CCCCC" LONG_FIELD3="DDDDDDDDDD"\n', str(self.writer))   


if __name__ == '__main__':
    unittest.main()

