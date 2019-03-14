"""
Read the XML layout definition and creates Record/Field objects.
"""

import os

from xml.dom import minidom

from rbf.element import Element
from rbf.fieldtype import FieldType
from rbf.field import Field
from rbf.record import Record

class Layout(Element):
    """
    Define a structure of a record-based file by reading
    the XML file description given as argument and load its description into a dictionary.
    This class is merely a dictionary of Record objects

    :param str xml_file: name of the file describing record file structure
    :raises ValueError:
        * if **xml_file** is not accessible

    ::

        >>> from rbf.layout import Layout
        >>> layout = Layout("world_data.xml")
        >>> layout.name
        'world_data.xml'
        >>> layout.description
        'Continents, countries, cities'
        >>> len(layout["CONT"])
        88
        >>> len(layout["COUN"])
        74
        >>> "FOO" in layout
        False
        >>> "CONT" in layout
        True

    """
    def __init__(self, xml_file: str):
        # check file if accessible
        if not os.path.isfile(xml_file):
            raise ValueError("XML description file {0} not found!!".format(xml_file))

        # call parent ctor
        super(self.__class__, self).__init__(xml_file, "", 0)

        # init record dict
        self._record = {}

        # init field type dict
        ftypes = {}

        # parse document
        doc = minidom.parse(xml_file)

        # get <meta> attributes
        meta = doc.getElementsByTagName("meta")[0]

        # automatically add meta tags
        self._add_meta_tags(meta)

        # build field types dict
        for node in doc.getElementsByTagName("fieldtype"):
            # create Field object and save it into dict
            ft = FieldType.from_xml_node(node)
            ftypes[ft.name] = ft

        # loop on all records
        for rec in doc.getElementsByTagName("record"):
            # create first rec object
            recname = rec.attributes['name'].value
            recdesc = rec.attributes['description'].value

            self._record[recname] = Record(recname, recdesc)

            # now loop on fields and append field to record
            for node in rec.childNodes:
                if node.nodeType == node.ELEMENT_NODE and node.nodeName == "field":
                    fname = node.attributes['name'].value
                    fdesc = node.attributes['description'].value
                    ftype = ftypes[node.attributes['type'].value]
                    flength = int(node.attributes['length'].value)

                    # add field to record
                    self._record[recname].append(Field(fname, fdesc, ftype, flength))

    def _add_meta_tags(self, meta):
        """ useful helper to add meta tags automatically

        :param meta: meta tag object when reading the <meta> XML tag from layout file
        """
        for tag_name in meta.attributes.keys():
            setattr(self, meta.attributes[tag_name].name, meta.attributes[tag_name].value)

    def __getitem__(self, key: str) -> Record:
        """ return the corresponding record

        :param key: name of the record to fetch
        :return: the matching record object
        :raises ValueError:
            * if **key** is not found
        """
        if key not in self._record:
            raise ValueError("key {0} not found in record {1}!!".format(key, self.name))
        return self._record[key]

    def __iter__(self):
        """ loop iterator """
        for k in sorted(self._record):
            yield self._record[k]

    def __contains__(self, key: str) -> bool:
        """
        :param key: name of the record
        :return: true is record name is found in record dict
        """
        return key in self._record

    def __str__(self) -> str:
        # as layout derives from Element, first call the super printer
        return str(self.__dict__)

    def records(self) -> dict:
        """ return record dictionary """
        return self._record

    def keep(self, recnames: list):
        """ keep only records in the given list

        :param list recnames: list of record names to keep in layout

        """
        self._record = { k:v for k,v in self._record.items() if k in recnames }

    def delete(self, recnames: list):
        """ delete only records in the given list

        :param list recnames: list of record names to delete from layout

        """
        self._record = { k:v for k,v in self._record.items() if k not in recnames }

    def prune(self, fnames: list):
        """ delete from all records the field  names in flist
        
        :param list flist: list of comma separated field names to delete from all records
        
        """
        for rec in self: 
            rec.delete(fnames)

    def simplify(self, data):
        """ delete record and fields not matching the given list

        :param str data: list of records and fields, like this:
            ["RECORD1:F2,F3,F4", "RECORD2:D5,D6"]

        """
        rec_list = []

        for s in data:
            # extract record name and list of fields
            (recname, fields) = s.split(':')
            field_list = [e.strip() for e in fields.split(',')]

            # build the list of record to keep to only keep them
            rec_list.append(recname.strip())

            # for each field, only keep those we want
            rec = self[rec_list[-1]]
            rec.keep(field_list)

        # now delete unwanted record
        self.keep(rec_list)


