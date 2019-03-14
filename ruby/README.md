# rbf, a record-based file library in Ruby

Record-based files are very popular on mainframes and in some industries. They are plain vanilla ASCII (or EBCDIC files but not the deal here) where each line corresponds to a record, and each record to fields.

So a rbf file is a set of records, and a record is a set of fields. Each record may have a type, usually defined by the first characters on the line. Each record is made of the concatenation of fields, each having its length and its position within a its parent record.

## Layout definition file

Such a file could be easily defined by an XML layout file. Suppose you've got an ascii file for some statistical data on continents, countries such as capital, population, etc. Such a file could look like this:

```
CONTAsia           43820000            16920000            29.5     Shanghai            
COUNChina                         1338100000          Beijing             
COUNChina Hong Kong SAR           7000000             Hong KongR
COUNChina Macau SAR               500000              Macau City          
COUNChina Tibet                   2620000             Lhasa               
COUNJapan                         127400000           Tokyo               
COUNKorea (North)                 22800000            P'yongyang          
COUNKorea (South)                 48900000            Seoul               
COUNMongolia                      2800000             Ulaanbaatar         
COUNTaiwan                        23200000            Taipei              
COUNRussian Federation            141900000           Moscow              
COUNAfghanistan                   29100000            Kabul               
COUNBangladesh                    164400000           Dhaka               
COUNBhutan                        700000              Thimphu             
COUNIndia                         1188800000          New Delhi           
COUNIran                          75100000            Tehran              
```

The corresponding XML definition file describing this format could be:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!-- inspired from https://en.wikipedia.org/wiki/List_of_continents_by_GDP_%28nominal%29 -->
<!-- and http://www.nationsonline.org/oneworld/asia.htm -->
<rbfile
    xmlns="http://www.w3schools.com"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.w3schools.com rbf.xsd"
>

    <meta version="1.0" description="Continents, countries, cities" ignoreLine="^#" skipField="ID" mapper="type:1 map:0..4"/>

	<fieldtype name="CHAR" type="string" pattern="\w+" format=""/>
	<fieldtype name="NUM" type="decimal"/>
	<fieldtype name="INT" type="integer"/>

	<record name="CONT" description="Continent data">
		<field name="ID" description="Record ID" length="4" type="CHAR"/>
		<field name="NAME" description="Name of the continent" length="15" type="CHAR"/>
		<field name="AREA" description="Area of the continent" length="20" type="NUM"/>
		<field name="POPULATION" description="Population of the continent" length="20" type="NUM"/>
		<field name="DENSITY" description="Density per km2" length="9" type="NUM"/>
		<field name="CITY" description="Most populus city" length="20" type="CHAR"/>
	</record>

	<record name="COUN" description="Country data">
		<field name="ID" description="Record ID" length="4" type="CHAR"/>
		<field name="NAME" description="Name of the country" length="30" type="CHAR"/>
		<field name="POPULATION" description="Number of inhabitants" length="20" type="INT"/>
		<field name="CAPITAL" description="Capital of the country" length="20" type="CHAR"/>
	</record>

</rbfile>
```

This ruby library is very simple to use, once you've got the XML definition file defined.

## Installation
Download the **lib** directory and set your **RUBYLIB** environment variable. 

## How to use it
Add the tradional `require "rbf"`to your Ruby file and starts playing

```ruby
require "rbf"

# first step: create a Layout object which reads the XML definition file and creates Record and Field objects
# to play with. A Layout object is a hash of record, with the key being the record name, and the value the
# Record object
layout = Layout.new("world_data.xml")

# now create a Reader object: it just read every line of the input record-based file and maps it to a Record object from
# the Layout hash. If the next line is of the same Record type, it will be overwritten.
# The 3rd argument is a function or a lambda mapping the line to a record type.
reader = Reader.new("world_data.txt", layout, lambda {|x| x[0..3]})

# now, read each record. Next line for example build a CSV file from the rbf input file **world_data.txt**
reader.each {|rec|
  puts rec.array_of(:@value).join(";") 
}
```
