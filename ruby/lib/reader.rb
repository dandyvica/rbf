# Use this class to read a record-based file and loop through each record.
#
# ==== Example
#
#   # import our lib (should be in your RUBYLIB env variable)
#   require 'rbf'
#
#   # open rbf file and convert it to a CSV (;) file
#   layout = Layout.new("world_data.xml")
#   reader = Reader.new("world_data.txt", layout, lambda {|x| x[0..3]})
#
#   # convert record to CSV (;-separated)
#   reader.each {|rec| puts rec.array_of(:@value).join(";") }
# 
class Reader

    # :call-seq:
    # new(rb_file, layout, mapper) -> new_Reader
    #
    # ==== Arguments
    #
    # * +rb_file+ - recrd-based file
    # * +layout+  - Layout object
    # * +mapper+  - a func or lambda returning the record ID from the line read
    def initialize(rb_file, layout, mapper)
        # check arguments
        raise ArgumentError, "input file #{rb_file} is not found!" unless File.exist?(rb_file)

        # save args
        @rb_file = rb_file
        @layout = layout
        @mapper = mapper

    end

    # iterator returning a record object
    # :call-seq:
    # each -> Record object
    def each
        File.open(@rb_file, "r").each_line {|line| 
            # first identify record from the line
            recname = @mapper.call(line)

            # recname in layout?
            if not @layout.rmap.include?(recname)
                STDERR.puts "record #{recname} not found!"
                next
            end

            # get record
            @layout[recname].value = line

            # return recname
            yield @layout[recname]
        }
    end

end

