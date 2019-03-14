# This class defines a field type. The list of possible field types are:
# :string, :decimal, :integer, :date, :time, :overpunch
#
# ==== Example
# 
#    ft = FieldType.new("A/N", "string")
#    ft.name        #=> "A/N"
#    ft.description #=> "string"
#    ft.length      #=> 0
#    ft.type        #=> :string
class FieldType < Element
    # the field type as a symbol
    attr_reader :type

    # list of possible values for a field type
    @@admissible_types = [:string, :decimal, :integer, :date, :time, :overpunch]

    # create a new field type
    # :call-seq:
    # new(data_type_representation, data_type_description) -> new_FieldType
    def initialize(data_type_representation, data_type_description)
        # use Element helper class
        super(data_type_representation, data_type_description, 0)

        # create the type of field as a sympol
        @type = data_type_description.to_sym

        # test admissible values for @type
        if !@@admissible_types.include?(@type)
            raise ArgumentError, "{data_type_description} is not an admissible type!"
        end
    end

    # convert overpunch value to ASCII regular value
    # :call-seq:
    # from_overpunch(from_str) -> str
    #
    # ==== Example
    # 
    #    ft = FieldType.new("N", "overpunch")
    #    ft.from_overpunch("1245AB")            #=> "124512"
    def from_overpunch(from_str)
        from_str.tr("{}ABCDEFGHIJKLMNOPQR", "00123456789123456789")
    end
end

