require "rbf"

# first step: create a Layout object which reads the XML definition file and creates Record and Field objects
# to play with. A Layout object is a hash of record, with the key being the record name, and the value the
# Record object
layout = Layout.new("../data/world/world_data.xml")

# now create a Reader object: it just read every line of the input record-based file and maps it to a Record object from
# the Layout hash. If the next line is of the same Record type, it will be overwritten.
# The 3rd argument is a function or a lambda mapping the line to a record type.
reader = Reader.new("../data/world/world_data.txt", layout, lambda {|x| x[0..3]})

# now, read each record. Next line for example build a CSV file from the rbf input file **world_data.txt**
reader.each {|rec|
  puts rec.array_of(:@value).join(";") 
}