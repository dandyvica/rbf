"""
Provide a class to manage related record-based file.
"""

class Element(object):
    """
    This is the base class for defining objects found in **record-based files** (rbf).
    It can be viewed as an atomic structure, a cinder block upon which 
    rbf data structures are based.

    :param str name: name of the element
    :param str description: description of the element
    :param int length: number of bytes of the element
    :raises ValueError: 
        * if **name** is empty
        * if **length** is negative

    ::

        >>> from rbf.element import Element
        >>> e = Element("ELEM1", "This is element #1", 5)
        >>> e
        name=<ELEM1> description=<This is element #1> length=<5>
        >>> e.name
        'ELEM1'
        >>> e.description
        'This is element #1'
        >>> e.length
        5
        >>> len(e)
        5

    """
    def __init__(self, name: str, description: str, length: int):
        # only test for a valid length and empty name
        # description could be empty
        if length < 0: raise ValueError("element length is negative!!")
        if name == "": raise ValueError("element name is empty!!")

        # boilerplate code
        self.name        = name
        self.description = description
        self.length      = length

        # used to print out text data. Useful to compute this at build time and not
        # at run time
        self.cell_length1 = max(length, len(name))
        self.cell_length2 = max(length, len(description), len(name))  

    def __eq__(self, other) -> bool:
        """ element equality """
        return self.name == other.name and self.description == other.description and self.length == other.length

    def __len__(self) -> int: 
        """ same as length property """
        return self.length

    def __repr__(self) -> str:
        return "name=<{0.name}> description=<{0.description}> length=<{0.length}>".format(self)

