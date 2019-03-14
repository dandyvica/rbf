# This is the base class for defining objects found in record-based files (rbf).
# It can be viewed as an atomic structure, a cinder block upon which 
# rbf data structures are based.
#
# ==== Example
# 
#    e = Element.new("ELEM1", "Description of element 1", 10)
#    e.name        #=> "ELEM1"
#    e.description #=> "Description of element 1"
#    e.length      #=> 10
class Element
    # the name of the element, usually a nickname to refer to
    attr_accessor :name

    # a long description of the element, more detailed than its nickname
    attr_accessor :description

    # the length (in bytes) of the element (0-length is allowed)
    attr_accessor :length

    # element constructor
    # :call-seq:
    # new(name, description, length) -> new_Element
    def initialize(name, description, length)
        @name, @description, @length = name, description, length
    end
end

