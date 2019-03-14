"""
Define a field type (to be used with the Field class).
"""

from xml.dom import minidom

from rbf.element import Element
from rbf.basetype import BaseType

class FieldType(BaseType):
    """
    Even if a field within a record-based file is nothing but an ASCII file, some fields within
    a record can represent numerical, alphumerical, etc type of data. This class holds the type
    of field.

    :param str data_type_representation: nickname of the field type
    :param str data_type_description: base type which is restricted to a list of admissible types

    ::

        >>> from rbf.fieldtype import FieldType
        >>> ft = FieldType("A/N", "string")
        >>> ft
        data_type_representation=<A/N> data_type_description=<string> type=<BaseType.string>
        >>> ft.name
        'A/N'
        >>> ft.description
        'string'
        >>> ft.type
        <BaseType.string: 'string'>

    """
    def __init__(self, name: str, type_of_string: str):
        # call parent ctor
        super(FieldType,self).__init__(type_of_string)
        self.name = name

    def convert(self, value):
        """
        convert a value according to its base type
        """
        if self.type_as_string == "date":
            return super().convert(value, self.date_format)
        elif self.type_as_string == "time":
            return super().convert(value, self.time_format)
        elif self.type_as_string in ['decimal','integer']:
            # sometimes, we need to get rid of leading 0 as conversion will fail if
            # a negative sign is found (e.g.: 00000-6) 
            s = value.lstrip("0")
            if s == "": 
                return super().convert("0")
            else:
                return super().convert(value.lstrip("0"))
        else:
            return super().convert(value)             

    @classmethod
    def from_xml_node(cls, xml_node):
        """
        second constructor to build a field type from an XML-string like <fieldtype name="A" type="string"/>

        :param str xml_node: XML-node object describing a field type

        """
        name           = xml_node.attributes['name'].value
        type_of_string = xml_node.attributes['type'].value

        # create class instance
        ft = cls(name, type_of_string) 
        
        # add any attribute we might find in the <fieldtype> tag
        # other than name or type tag XML attribute
        for tag_name in [t for t in xml_node.attributes.keys() if t not in ['name', 'type']]:
            setattr(ft, tag_name, xml_node.attributes[tag_name].value)

        return ft

    def __repr__(self) -> str:
        return "name=<{0.name}> type_as_string=<{0.type_as_string}> type=<{0.basic_type}>".format(self)

