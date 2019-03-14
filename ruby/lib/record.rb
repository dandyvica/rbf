# A record is a set of fields found in a record-based file
#
# ==== Example
#
#   # create new record
#   r = Record.new("RECORD1", "Description of record 1")
#
#   # add field objects
#   r.push(Field.new("FIELD1", "Description of field 1", FieldType.new("A", "string"), 10))
#   r.push(Field.new("FIELD2", "Description of field 2", FieldType.new("N", "integer"), 5))
#   r.push(Field.new("FIELD2", "Description of field 2", FieldType.new("N", "integer"), 5))
#   r.push(Field.new("FIELD3", "Description of field 3", FieldType.new("A", "string"), 10))
#
#   # access field by index
#   f1 = r[0]           #=> first field in the record
#
#   # access by field name
#   f2s = r["FIELD2"]   #=> array of Field objects whose name is "FIELD2"
#   f2s.size            #=> 3
#   
#   # test if field is in record
#   r.include?("FIELD1")   #=> true
#   r.include?("FOO")      #=> false
#
#   # get record data
#   r.size                 #=> 4
#   r.length               #=> 30
#
#   # set record value by setting all fields values
#   s = "AAAAAAAAAA0000100002BBBBBBBBBB"
#   r = s
#   r[0].value             #=> "AAAAAAAAAA"
#   r[1].value             #=> "00001"
#   r[2].value             #=> "00002"
#   r[3].value             #=> "BBBBBBBBBB"
#
#   # and build array of attributes
#   r.array_of(:@name)     #=> ["FIELD1","FIELD2","FIELD2","FIELD3"]
#   r.array_of(:@length)   #=> [10, 5, 5, 10]
#
#   # iterate on all fields
#    r.each {|field| puts field.name }
#
class Record < Element
    include Enumerable
    
   # attr_reader :flist, :fmap

    # create a record
    # :call-seq:
    # new(name, description) -> new_Record
    def initialize(name, description)
        # record name should not be empty
        raise ArgumentError, "record name is empty!" unless name != ""

        super(name, description, 0)

        # initialize array and hash which will store field objects
        @flist = Array.new
        @fmap = Hash.new {|h,k| h[k] = Array.new }
    end

    # Append a field into a record. At the same time, update field lower/upper bounds and other field
    # data only relevant when inside a record
    #
    # :call-seq:
    # push(Field) -> ary
    #
    # ==== Arguments
    #
    # * +field+ - a Field class object
    def <<(field)
        field.index = @flist.count
        field.offset = @length 

        # save field in a list, and in a hash because field might have the same name. Using the hash,
        # it's easier to retrieve
        @flist.push(field)
        @fmap[field.name].push(field)

        # at the same time, build alternate field name
        field.alternate_name = field.name + @fmap[field.name].size.to_s

        # as we add a field, record length is growing
        @length += field.length

        # and adjust field bounds
        field.lower_bound = field.offset
        field.upper_bound = field.offset + field.length
    end

    # Returns the number of fields in the record
    # :call-seq:
    # count -> fixnum
    def count
        @flist.count
    end

    # Field  access with either a string (field name) or an integer (field index). If a field name is passed,
    # as fields could have the same name, an array of Field objects is returned.
    #
    # :call-seq:
    # [](key) -> Field or ary of Field objects
    #
    # ==== Arguments
    #
    # * +key+ - return the field object matching field name (if key is a string), or its index in the record
    # if key is an integer
    #
    # === Raise
    #
    # +ArgumentError+ if:
    # * key (string) is not found
    # * key (integer) is out of bounds
    # * key is neither a string nor an integer
    def [](key)
        if key.is_a?(String)
            if !@fmap.include?(key) 
                raise ArgumentError, "key #{key} is not found in record #{name}!"
            end
            @fmap[key]
        elsif key.is_a?(Integer)
            if key < 0 or key >= self.count
                raise ArgumentError, "key #{key} is out of bounds in record #{name}!"
            end
            @flist[key]
        else
            raise ArgumentError, "key #{key} is not a string nor an integer for record #{name}!"
        end
    end

    # iterator: iterates through each field
    def each(&block)
        @flist.each(&block)
    end

    # set record value by setting all field values
    def value=(s)
        # adjust length to exactly fit record length
        if s.length < @length
            s = s.ljust(@length)
        elsif s.length > @length
            s = s[0...@length]
        end

        # set each field value because we already know field bounds
        self.each {|f| f.value = s[f.lower_bound...f.upper_bound] }

    end

    # return true if a field name is found in the record
    # :call-seq:
    # include?(fname) -> bool
    #
    # ==== Arguments
    #
    # +fname+ - field name
    def include?(fname)
        @fmap.include?(fname)
    end

    # build an array of attributes data for the argument
    # :call-seq:
    # array_of(attr) -> ary
    def array_of(attr)
        @flist.map {|x| x.instance_variable_get(attr) }
    end

    # return the record value, which is just the concatenation of all field raw values.
    # Returned string length is equal to record length
    #
    # :call-seq:
    # value -> str
    #
    def value
        self.array_of(:@raw_value).join
    end
end
