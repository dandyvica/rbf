"""
Define a way to represent records as read from a record-base file.
"""
import os

from rbf.field import Field
from rbf.record import Record
from rbf.layout import Layout

class Reader(object):
    """
    create a representation of a record-based file

    :param str rb_file: file name to read (should match the format described by the **Layout** object)
    :param Layout layout: Layout object
    :param lambda mapper: function which identify the record key in the record list from the current line

    ::

        >>> from rbf.layout import Layout
        >>> from rbf.reader import Reader
        >>> layout = Layout("world_data.xml")
        >>> reader = Reader("world_data.txt", layout, lambda x: x[0:4])

    """

    def __init__(self, rb_file: str, layout: Layout, mapper):
        # verify arguments
        if not os.path.isfile(rb_file):
            raise ValueError("Input file {0} not found!!".format(rb_file))

        # save members
        self._layout  = layout
        self._mapper  = mapper
        self._rb_file = rb_file

    def _map_record_from_line(self, line: str) -> str:
        """

        return a record object from the line read from file

        :param str line: string read from file

        """
        # try to discover record name from line read
        record_name = self._mapper(line)

        # but we don't find it! stop or just skip the line?
        if record_name not in self._layout:
            return None

        # record is found in read line
        return self._layout[record_name]

    def __iter__(self):
        """
        iterator to read each line of the record-based file and return the matching Record object

        ::

            # then loop
            for rec in reader:
                print(rec)


        """
        # read file line by line
        with open(self._rb_file, 'r') as fh:
            for line in fh:
                # strip out \n
                line = line.rstrip("\n")

                # get record if any
                rec = self._map_record_from_line(line)

                # rec not found: just loop
                if not rec:
                    continue

                # set line value
                rec.value = line

                # return record to our "for" loop
                yield rec

    def by_record_name(self, recname_list: list):
        """ 
        iterator to only fetch records which name is matching the **recname_list**
        
        """
        for rec in self:
            if rec.name in recname_list: 
                yield rec

