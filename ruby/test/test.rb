require "rbf"

require "test/unit"


class TestRbf < Test::Unit::TestCase

    def test_element
        e = Element.new("FIELD1", "Description of field 1", 10)
        assert_equal(e.name, "FIELD1")
        assert_equal(e.description, "Description of field 1")
        assert_equal(e.length, 10)
    end

    def test_fieldtype
        ft = FieldType.new("A/N", "string")
        assert_equal(ft.type, :string)
        assert_raise(ArgumentError) { FieldType.new("C", "complex") }
    end

    def test_field
        string_type = FieldType.new("A/N", "string")
        f = Field.new("FIELD1", "Description of field 1", string_type, 10)

        f.value = "     AAAAAAAAAAAAAAAA "
        assert_equal(f.type.type, :string)
        assert_equal(f.value, "AAAAAAAAAAAAAAAA")

        ov_type = FieldType.new("N", "overpunch")
        f = Field.new("FIELD1", "Description of field 1", ov_type, 15)
        f.value = "     123A "
        assert_equal(f.value, "1231")
    end

    def test_record
        assert_raise(ArgumentError) { Record.new("", "Description of field 1", "A/N", 10) }

        string_type = FieldType.new("A/N", "string")
        f1 = Field.new("FIELD1", "Description of field 1", string_type, 10)
        f2 = Field.new("FIELD2", "Description of field 2", string_type, 20)
        f3 = Field.new("FIELD3", "Description of field 3", string_type, 10)
        f4 = Field.new("FIELD4", "Description of field 4", string_type, 5)
        f5 = Field.new("FIELD5", "Description of field 5", string_type, 5)
        r = Record.new("RECORD1", "Description of record 1")

        assert_equal(r.name, "RECORD1")

        r << f1
        r << f2
        r << f3
        r << f4
        r << f5

        assert_equal(r.include?("FIELD1"), true)

        assert_equal(r.count, 5)
        assert_equal(r.length, 50)

        assert_raise(ArgumentError) { p r[14.3] }
        assert_raise(ArgumentError) { p r["XXX"] }
        assert_raise(ArgumentError) { p r[-1] }
        assert_raise(ArgumentError) { p r[5] }
        assert_equal(r[2], f3)
        assert_equal(r["FIELD3"][0], f3)

        r.value = "AAAAAAAAAABBBBBBBBBBBBBBBBBBBBCCCCCCCCCCDDDDDEEEEE"
        assert_equal(f1.value, "AAAAAAAAAA")
        assert_equal(f2.value, "BBBBBBBBBBBBBBBBBBBB")
        assert_equal(f3.value, "CCCCCCCCCC")
        assert_equal(f4.value, "DDDDD")
        assert_equal(f5.value, "EEEEE")

        assert_equal(r.array_of(:@name), ["FIELD1","FIELD2","FIELD3","FIELD4","FIELD5"])    
        assert_equal(r.value, "AAAAAAAAAABBBBBBBBBBBBBBBBBBBBCCCCCCCCCCDDDDDEEEEE")

        r.each {|f| p f }
        #r.each_with_index {|f,i| p i }
    end

    def test_layout
        assert_raise(ArgumentError) { Layout.new("foo.xml") }
        l = Layout.new("world_data.xml")
        r = l["COUN"]

        assert_equal(r.name, "COUN")

        l.each {|k,v| print v.name}
    end

    def test_reader
        assert_raise(ArgumentError) { Reader.new("foo.data") }

        l = Layout.new("world_data.xml")
        r = Reader.new("world_data.txt", l, lambda {|x| x[0..3]})

        r.each {|rec| print rec.name }
    end

end
