require 'rexml/document'
include REXML

# This class is a representation of all records and classes composing a record-based file.
# As a convention, the layout is defined is an XML file, with the following convention:
#
# * root tag is <rbfile>
# * field type tag (<fieldtype> XML tag)
# * record tag with the following attributes: name & description (<record> XML tag)
# * field tag inside a record tag (<field> XML tag)
#
# Suppose you get the following layout file (world_data.xml) describing a record-based file
# listing continent data:
#
#   <?xml version="1.0" encoding="UTF-8"?>
#   <!-- inspired from https://en.wikipedia.org/wiki/List_of_continents_by_GDP_%28nominal%29 -->
#   <!-- and http://www.nationsonline.org/oneworld/asia.htm -->
#   <rbfile
#       xmlns="http://www.w3schools.com"
#       xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
#       xsi:schemaLocation="http://www.w3schools.com rbf.xsd"
#   >
#
#       <meta version="1.0" description="Continents, countries, cities" ignoreLine="^#" skipField="ID" mapper="type:1 map:0..4"/>
#
#       <fieldtype name="CHAR" type="string" pattern="\w+" format=""/>
#       <fieldtype name="NUM" type="decimal"/>
#       <fieldtype name="INT" type="integer"/>
#
#       <record name="CONT" description="Continent data">
#           <field name="ID" description="Record ID" length="4" type="CHAR"/>
#           <field name="NAME" description="Name of the continent" length="15" type="CHAR"/>
#           <field name="AREA" description="Area of the continent" length="20" type="NUM"/>
#           <field name="POPULATION" description="Population of the continent" length="20" type="NUM"/>
#           <field name="DENSITY" description="Density per km2" length="9" type="NUM"/>
#           <field name="CITY" description="Most populus city" length="20" type="CHAR"/>
#       </record>
#
#       <record name="COUN" description="Country data">
#           <field name="ID" description="Record ID" length="4" type="CHAR"/>
#           <field name="NAME" description="Name of the country" length="30" type="CHAR"/>
#           <field name="POPULATION" description="Number of inhabitants" length="20" type="INT"/>
#           <field name="CAPITAL" description="Capital of the country" length="20" type="CHAR"/>
#       </record>
#
#   </rbfile>
#
# ==== Example
#
#      # create a layout object from XML definition file layout.xml
#      layout = Layout.new("world_data.xml")
#
#      # get continent data record
#      cont = layout["CONT"]
#      cont.size                #=> 6
#      cont[0].type.type        #=> :string
#
#  
class Layout < Element
    include Enumerable

    # hash of records, key is record name, value is Record object
    attr_reader :rmap

    # build fields & records from XML layout file
    # :call-seq:
    # new(xml_file) -> new_Layout
    def initialize(xml_file)
        # check file existence
        raise ArgumentError, "xml layout file #{xml_file} is not found!" unless File.exist?(xml_file)

        # initialize hash of FieldType objects
        ft_map = Hash.new

        # open document
        doc = Document.new(File.new(xml_file))
        root = doc.root

        # call parent ctor
        super(xml_file, root.attributes["description"], root.attributes["reclength"])

        # initialize record list
        @rmap = Hash.new

        # create all field types
        doc.elements.each("rbfile/fieldtype") {|ft|
            ft_map[ft.attributes["name"]] = FieldType.new(ft.attributes["name"], ft.attributes["type"])
        }

        # read record tags
        doc.elements.each("rbfile/record") {|e| 
            # create record
            recname = e.attributes["name"]
            @rmap[recname] = Record.new(recname, e.attributes["description"])

            # now add fields
            e.elements.each {|f| 
                field_type = ft_map[f.attributes["type"]]
                @rmap[recname] << 
                    Field.new(f.attributes["name"], f.attributes["description"], field_type, 
                              f.attributes["length"].to_i)
            }          
        }
    end

    # iterator on records
    def each(&block)
        @rmap.each_value(&block)
    end

    # access to the Record object matching record name
    # :call-seq:
    # [](recname) -> Record object
    def [](recname)
        @rmap[recname]
    end

end



