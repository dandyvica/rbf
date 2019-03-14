import os
import sys
import io

from rbf.field import Field
from rbf.record import Record
from rbf.writer.basewriter import Writer

"""

set of methods used to print out record data as text

"""


class TextWriter(Writer):

    """

    create a new text writer object

    :param output: text file name for output
    :type output: str

    :example:
    ::

            textwriter = TextWriter("myfile.txt")


    """
    def to_csv(self, rec, show_value=True, show_name=False):
        if show_name:
            self._fh.write(";".join(f.name for f in rec)+"\n")
        if show_value:
            self._fh.write(";".join(f.value for f in rec)+"\n")

    def write(self, rec):
        """

        write a record as an ASCII table, with field names as headers

        :param rec: record object
        :type rec: a Record object

        :example:
        ::

            writer.write(rec)


        """
        headers = "|".join([f.name.ljust(f.cell_length1) for f in rec]) + "\n"
        hrule = "-"*(len(headers)+2) + "\n"
        values = "|".join([f.value.ljust(f.cell_length1) for f in rec]) + "\n"

        self._fh.write(headers)
        self._fh.write(hrule)
        self._fh.write(values+"\n")

    def to_tag(self, rec):
        """

        write a record with all fields like FIELD="value"

        :param rec: record to write
        :type rec: Record object
        """
        self._fh.write(rec.name+":")
        self._fh.write(" ".join(["{0}=\"{1}\"".format(f.name, f.value) for f in rec])+"\n")

    def __str__(self) -> str:
        s = self._fh.getvalue()
        
        # reset buffer
        self._fh.close()
        self._fh = io.StringIO()

        return s
    
    def close(self):
        self._fh.close()
