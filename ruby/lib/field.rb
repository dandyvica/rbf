# A field is the tiniest piece of information found in a record-based file
#
# ==== Example
# 
#    f = Field.new("FIELD1", "Description of field 1", FieldType.new("A", "string"), 10)
class Field < Element
    # offset (in bytes) from the beginning of the record in which field is found (starts from 0)
    attr_accessor :offset

    # index of the field within its record
    attr_accessor :index

    # lower offset of the field within a record (starting byte position of the field, starting with for the first field)
    attr_accessor :lower_bound

    # upper offset of the field (ending position of the field)
    attr_accessor :upper_bound

    # when a field name is duplicated, it comes in handy to refer to that field
    # with a unique name. This alternate name is built by just adding it's index
    # to the field name
    attr_accessor :alternate_name

    # field type as defined in the layout definition file
    attr_reader :type

    # when set, the blank stripped value of the field
    attr_reader :value

    # when set, the blank non-stripped value of the field (original value)
    attr_reader :raw_value

    # build a new field
    # :call-seq:
    # new(name, description, type, length) -> new_Field
    def initialize(name, description, type, length)
        super(name, description, length)

        @type = type
    end


    # Store a value inside a field. Both blank stripped value and orignal value are stored.
    #
    # ==== Arguments
    #
    # * +s+ - string value to store inside the field. Usually, comes from a text file
    #
    # ==== Example
    # 
    #    f.value = "   123456    "
    #    f.value       #=> "123456"
    #    f.raw_value   #=> "   123456    "
    def value=(s)
        @value = s.strip
        @raw_value = s

        # in case of overpunched value, just convert it
        if @type.type == :overpunch 
            @value = type.from_overpunch(@value)
        end
    end

end
