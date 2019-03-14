#-----------------------------------------------------------------------------------
# RBF Python3 library example.
#
# reads a RBF file and print-out record as CSV data
#
#-----------------------------------------------------------------------------------

# standard imports
import sys
from datetime import datetime, date, time

# import our lib
import rbf

# test arguments
if len(sys.argv) == 1:
    print("Usage: %s <xml layout> <rbf file>\n" % sys.argv[0])
    sys.exit()

# save arguments
xml_layout = sys.argv[1]
rb_file = sys.argv[2]

# create new mapper for layout object. It is necessary to define this map to read a RBF file
mapper = lambda x: x[0:4]

# create new layout
layout = rbf.Layout(xml_layout)

# create new reader
reader = rbf.Reader(rb_file, layout, mapper)

# now read each record
for rec in reader:
    print(";".join(rec.array_of('value')))


