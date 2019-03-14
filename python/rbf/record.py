"""
Defines a generic record made of fields and allow easy construction and manipulation of
underlying fields.

"""

from rbf.element import Element
from rbf.field import Field

class Record(Element):
    """
    Define a new record based on name and description.

    :param str name: name of the record
    :param str description: description of the record
    
    ::

        >>> from rbf.fieldtype import FieldType
        >>> from rbf.field import Field
        >>> from rbf.record import Record
        >>> rec = Record("RECORD1", "Description for record #1")
        >>> rec.name
        'RECORD1'
        >>> rec.description
        'Description for record #1'
        >>> ft = FieldType("A/N", "string")
        >>> rec.append(Field("FIELD1", "Description of field 1", ft, 10))
        >>> rec.append(Field("FIELD2", "Description of field 2", ft, 5))
        >>> rec.append(Field("FIELD2", "Description of field 2", ft, 5))
        >>> rec.append(Field("FIELD3", "Description of field 3", ft, 10))
        >>> rec.count()
        4
        >>> len(rec)
        30
        >>> rec[0]
        name=<FIELD1> description=<Description of field 1> length=<10> data_type_representation=<A/N> data_type_description=<string> type=<BaseType.string> value=<> raw_value=<> offset=<0> index=<0> bounds=<0:10>
        >>> rec["FIELD2"]
        [name=<FIELD2> description=<Description of field 2> length=<5> data_type_representation=<A/N> data_type_description=<string> type=<BaseType.string> value=<> raw_value=<> offset=<10> index=<1> bounds=<10:15>, name=<FIELD2> description=<Description of field 2> length=<5> data_type_representation=<A/N> data_type_description=<string> type=<BaseType.string> value=<> raw_value=<> offset=<15> index=<2> bounds=<15:20>]
        >>> len(rec["FIELD2"])
        2
        >>> rec.value = "AAAAAAAAAABBBBBCCCCCDDDDDDDDDD"
        >>> rec.FIELD1
        'AAAAAAAAAA'
        >>> len(rec.FIELD1)
        10
        >>> rec[2].value
        'CCCCC'
        >>> rec.array_of('name')
        ['FIELD1', 'FIELD2', 'FIELD2', 'FIELD3']
        >>> rec.array_of('value')
        ['AAAAAAAAAA', 'BBBBB', 'CCCCC', 'DDDDDDDDDD']
        >>> "FIELD1" in rec
        True
        >>> "FOO" in rec
        False

    """

    def __init__(self, name: str, description: str):
        # call parent class ctor
        super(self.__class__, self).__init__(name, description, 0)

        # will hold field objects as a list of fields and a dict whose
        # key is the field name and value a list of field objects
        self._field_list = []
        self._field_dict = {}

    @property
    def value(self):
        """

        the value of the record

        :getter: record value = string concatenation of all field values
        :setter: set the record value (= all field values) from its argument
        :type: str

        """
        return "".join(self.array_of('raw_value'))

    @value.setter
    def value(self, new_value):
        """
        set the value for this record by setting the value of all fields of this record

        """
        # test line length: if longer, cut it
        if len(new_value) > self.length:
            new_value = new_value[0:self.length]
        # or if longer strip it
        elif len(new_value) < self.length:
            new_value = new_value.ljust(self.length)

        # setting record value is setting value for all fields/records composing the record
        for field in self:
            # field value is a slice of initial string
            field.value = new_value[field.lower_bound:field.upper_bound]

    def __getitem__(self, key):
        """
        access to a field by key

        :param key: could be an int or slice (standard list item access) or a field which
                gives back list of Field objects matching name
        :type key: int or slice or str

        ::

            field = rec["FIELD1"]
            field = rec[1]
            field = rec[2:4]

        :warning: a record may contain fields having the same name. So
            accessing by name using [] returns an array of field objects (the same with a slice)

        :todo: check slice bounds
        :rtype: list or str

        """
        if type(key) == int or type(key) == slice:
            return self._field_list[key]
        elif type(key) == str:
            if key not in self._field_dict: 
                raise ValueError("key {0} not found in record {1}".format(key, self.name))
            return [f for f in self._field_list if f.name == key]
        else:
            raise TypeError("Object {0} is of type {1} and not in int or str".format(key, type(key)))

    def __contains__(self, key: str) -> bool:
        """
        test if given field name belongs to record

        :param str key: field name to check existence for

        ::

            if "FIELD1" in rec:
                print("FIELD1 is found!!")

        """
        return key in self._field_dict

    def __repr__(self) -> str:
        output = format(super(self.__class__, self).__repr__()) + "\n"
        for field in self:
            output += "\t{0}\n".format(field)
        return output

    def __iter__(self):
        """
        as the field list is held using a regular list, just
        returns its iterator

        ::

            for field in rec:
                print(field)


        """
        return iter(self._field_list)

    def __getattr__(self, name: str) -> str:
        """
        return the value of the field name

        :param str name: field name
        :raises ValueError: 
            * if **name** is not found in record
            * if **name** field occurence is > 1

        ::

            if rec.FIELD1 == "My value":
                print("FIELD1 value is ok!!")

        """
        if name not in self:
            raise ValueError("attribute name '{0}' not found in record {1}".format(name, self.name))

        # depending on whether it's a Field or a Record
        items = self[name]

        # more than 1 item? impossible to fetch something, too ambiguous
        if len(items) > 1:
            raise ValueError("Ambiguous attribute name {0}".format(name))

        return items[0].value

    def append(self, field: Field):
        """
        insert (at the end, so append) a field object into a record

        :param Field field: field object to append

        ::

            rec.append(Field("FIELD1", "Field description #1", FieldType("A","string"), 10))

        """

        # set field index at the same time
        field.index = len(self._field_list)

        # offset at this moment is merely the length of record (starts at 0)
        field.offset = self.length

        # add a field to record
        self._field_list.append(field)

        # as a field name can be appear more than once, we keep
        # track of all fields using this dict
        if field.name in self._field_dict:
            self._field_dict[field.name].append(field)
        else:
            self._field_dict[field.name] = [field]

        # record is becoming longer
        self.length += field.length

        # and adjust field bounds
        field.lower_bound = field.offset
        field.upper_bound = field.offset + field.length

    def count(self) -> int:
        """ return the number of fields in the record """
        return len(self._field_list)

    def array_of(self, attr_name: str) -> list:
        """ 
        build an array of field attributes data for the argument

        :param str attr_name: attribute name for building the list
        :raises ValueError: if **attr_name** is not a Field class attribute

        """
        if not hasattr(self[0], attr_name): 
            raise ValueError("Field class has no attribute named <{0}>!!".format(attr_name))

        return [getattr(f, attr_name) for f in self]

    def asdict(self, convert=True) -> dict:
        """
        build a dict where keys are field name and values are field value or typed_value
        for all fields composing the record

        :param bool convert: if True, conversion is done from string value to typed value
        """
        d = {}

        # loop through fields because insertion order is more useful
        for f in self:
            # do not store empty values
            if f.value == "": continue

            # try to convert if conversion is  needed
            if convert:
                converted_value = f.convert()
            else:
                converted_value = value

            # we already added a key: so need to change it to a list
            if f.name in d:
                if type(d[f.name]) == list:
                    d[f.name].append(converted_value)
                else:
                    old_value = d[f.name]
                    d[f.name] = [old_value, converted_value]
            else:
                d[f.name] = converted_value

        return d

    def delete(self, fname_list: list, stringent: bool = False):
        """
        delete all fields matching names passed in the **fname_list** list

        :param list fnames: list of field names to delete

        """
        for fname in fname_list:
            self._field_list = [f for f in self._field_list if f.name != fname]

            # don't care if field name is not found
            if stringent:
                del self._field_dict[fname]
            else:
                self._field_dict.pop(fname, None)
    
    def keep(self, fname_list: list):
        """ keep only the fields specified in the list

        :param list fnames: list of field names to keep. Others are deleted

        """
        self._field_dict = { k:v for k,v in self._field_dict.items() if k in fname_list }
        self._field_list = [f for f in self._field_list if f.name in fname_list]
    


