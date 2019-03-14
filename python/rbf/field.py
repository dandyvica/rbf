"""
Provide the Field class to manage text fields within a record-based file.

 """

from rbf.element import Element
from rbf.fieldtype import FieldType

class Field(Element):
    """Define a data field with its name, description, type and length

    This class should be used with its companion class **Record**. If a record can
    be mapped to a line of text within a file, then a field is a substring from
    that line, with a fixed length.

    Each field is holding the substring in the **value** and **raw_value** properties.

    :param str name: name of the field
    :param str description: description of the field
    :param FieldType fieldtype: format of the field (type of data found in the field)
    :param int length: number of bytes of the field

    ::

        >>> from rbf.fieldtype import FieldType
        >>> from rbf.field import Field
        >>> f = Field("FIELD1", "This is field #1", FieldType("A/N","string"), 5)
        >>> f
        name=<FIELD1> description=<This is field #1> length=<5> data_type_representation=<A/N> data_type_description=<string> type=<BaseType.string> value=<> raw_value=<> offset=<0> index=<0> bounds=<0:0>
        >>> f.name
        'FIELD1'
        >>> f.description
        'This is field #1'
        >>> f.length
        5
        >>> f.type
        data_type_representation=<A/N> data_type_description=<string> type=<BaseType.string>
        >>> f.type.type
        <BaseType.string: 'string'>
        >>> f.offset
        0
        >>> f.index
        0
        >>> f.lower_bound
        0
        >>> f.upper_bound
        0
        >>> f.value=" 45 "
        >>> f.value
        '45'
        >>> f.raw_value
        ' 45 '
    """
    def __init__(self, name: str, description: str, field_type: FieldType, length: int):
        # call parent class ctor
        super(self.__class__, self).__init__(name, description, length)

        # boilerplate code
        self.ftype = field_type

        # raw value is not stripped
        self.raw_value = self.str_value = ""

        # those attributes will be set when adding field into a record
        self.offset = self.index = self.lower_bound = self.upper_bound = 0

    """
    @classmethod
    def from_xml_node(cls, xml_node):
        second constructor to build a field from an XML-string like <field name="FIELD" description="Field desc" length="15" type="A"/>

        :param str xml_node: XML-node object describing a field type

        fname   = xml_node.attributes['name'].value
        fdesc   = xml_node.attributes['description'].value
        ftype   = ftypes[xml_node.attributes['type'].value]
        flength = int(xml_node.attributes['length'].value)

        # create class instance
        f = cls(fname, fdesc, ftype, flength)

        return f
    """

    @property
    def value(self):
        """
        * the stripped value as the field
        * property (r/w) -> str
        """
        return self.str_value

    @value.setter
    def value(self, s):
        """ set value for this field """
        # copy value as is
        self.raw_value = s

        # strip out blanks from string
        self.str_value = s.strip()

    def convert(self):
        """
        convert value attribute stored as string to a converted scalar value according to its type
        """
        return self.ftype.convert(self.value)

    def initialize(self):
        """
        initialize field value to its initial value as set by field type

        """
        self.reset(self.ftype.init)

    def reset(self, new_value):
        """
        format the **new_ value** argument as a string according to the field type format

        :param str new_value: value to format

        """
        fmt = self.ftype.format.replace('*', str(self.length))
        self.value = fmt % new_value

    def __eq__(self, other) -> bool:
        """ Field equality """
        return super(Field, self).__eq__(other) and self.ftype == other.ftype

    def __repr__(self) -> str:
        #return "{0} {1} value=<{2}> raw_value=<{3}> offset=<{4}> index=<{5}> bounds=<{6}:{7}>".format(super(self.__class__, self).__repr__(), \
        #         self.ftype, self.value, self.raw_value, self.offset, self.index, self.lower_bound, self.upper_bound)
        return "{0} {1.ftype} value=<{1.value}> raw_value=<{1.raw_value}> offset=<{1.offset}> index=<{1.index}> bounds=<{1.lower_bound}:{1.upper_bound}>" \
                .format(super(self.__class__, self).__repr__(), self)

