#include <iostream>
#include <cassert>
//#include <cppunit/extensions/HelperMacros.h>


#include <pugixml.hpp>

using namespace std;
using namespace pugi;

#include <rbf.h>
using namespace rbf;

void test_element();
void test_field_type();
void test_field();
void test_record1();
void test_layout();
void test_reader();

string xmlfile;
string rbffile;

int main(int argc, char**argv)
{
    if (argc == 1)
    {
        xmlfile = "./test/world_data.xml";
        rbffile = "./test/world_data.txt";
    }
    else
    {
        xmlfile = string(argv[1]);
        rbffile = string(argv[2]);

        //auto layout = Layout(xmlfile);
        Layout layout{xmlfile, 200};
        //auto reader = Reader(rbffile, layout, [](string s) { return s.substr(0,2); });
        //auto reader = Reader(rbffile, layout, [](string s) { return "JAF20A"; });
        Reader reader(rbffile, layout, [](string s) { return "JAF20A"; });

        for (auto &rec: reader)
        {
            //cerr << rec->value(';') << endl;
        }
        exit(1);
    }

    try 
    {
        // test Element class
        cout << "------------------------------------------------------------------" << endl;
        cout << "Testing test_element" << endl;
        test_element();

        // test fieldtype class
        cout << "------------------------------------------------------------------" << endl;
        cout << "Testing test_field_type" << endl;
        test_field_type();

        // test field class
        cout << "------------------------------------------------------------------" << endl;
        cout << "Testing test_field" << endl;
        test_field();

        // test Record class
        cout << "------------------------------------------------------------------" << endl;
        cout << "Testing test_record1" << endl;
        test_record1();

        // test layout
        cout << "------------------------------------------------------------------" << endl;
        cout << "Testing test_layout" << endl;
        test_layout();

        // test reader
        cout << "------------------------------------------------------------------" << endl;
        cout << "Testing test_reader" << endl;
        test_reader();
    }
    catch (std::exception& e) 
    {
        cout << e.what() << endl;
    }
}
void test_element()
{
    DataElement e0;

    auto e1 = Element<size_t>("ELEMENT_1", "Description for element 1", 10);
    auto e2(e1);
    auto e3 = e2;

    assert(e0.name().empty());
    assert(e0.description().empty());
    assert(e0.length() == 0);

    assert(e1.name() == "ELEMENT_1");
    assert(e1.description() == "Description for element 1");
    assert(e1.length() == 10);

    e1.setName("ELEMENT_2");
    e1.setDescription("Description for element 2");
    e1.setLength(50);

    assert(e1.name() == "ELEMENT_2");
    assert(e1.description() == "Description for element 2");
    assert(e1.length() == 50);

    assert(e2.name() == "ELEMENT_1");
    assert(e2.description() == "Description for element 1");
    assert(e2.length() == 10);

    assert(e3.name() == "ELEMENT_1");
    assert(e3.description() == "Description for element 1");
    assert(e3.length() == 10);

    assert(e1 != e2);
    assert(e2 == e3);
}

void test_field_type()
{
    FieldType ft0;
    auto ft1 = FieldType("A/N", "string");
    auto ft2(ft1);
    auto ft3 = ft2;

    assert(ft0.name().empty());
    assert(ft0.description().empty());
    assert(ft0.data_type() == DataType::VOID);

    assert(ft1.name() == "A/N");
    assert(ft1.description() == "string");
    assert(ft1.data_type() == DataType::STRING);

    assert(ft2.name() == "A/N");
    assert(ft2.description() == "string");
    assert(ft2.data_type() == DataType::STRING);

    assert(ft3.name() == "A/N");
    assert(ft3.description() == "string");
    assert(ft3.data_type() == DataType::STRING);

    assert(ft0 != ft1);
    assert(ft1 == ft2);
    assert(ft2 == ft3);
}

void test_field()
{
    Field f0;
    auto ft1 = FieldType("A/N", "string");
    auto f1 = Field("FIELD_1", "Field1 description", ft1, 15);
    auto f2(f1);
    auto f3 = f2;

    assert(f1.type().name() == "A/N");
    f1.setValue("    value1    ");
    assert(f1.value() == "value1");
    assert(f1.raw_value() == "    value1    ");

    assert(f1 == f2);
    assert(f2 == f3);

}
void test_record1()
{
    auto ft1 = FieldType("A/N", "string");
    auto f0 = Field("FIELD_0", "Field desc 0", FieldType("0", "string"), 10);
    auto f1 = Field("FIELD_1", "Field desc 1", FieldType("1", "string"), 10);
    auto f2 = Field("FIELD_2", "Field desc 2", FieldType("2", "string"), 10);
    auto f3 = Field("FIELD_3", "Field desc 3", FieldType("3", "string"), 10);
    auto f4 = Field("FIELD_4", "Field desc 4", FieldType("4", "string"), 10);

    f0.setValue("AAAAAAAAAA");
    f1.setValue("BBBBBBBBBB");
    f2.setValue("CCCCCCCCCC");
    f3.setValue("DDDDDDDDDD");
    f4.setValue("EEEEEEEEEE");

    auto rec = Record("RECORD1", "Desc for record 1");
    assert(rec.name() == "RECORD1");
    assert(rec.description() == "Desc for record 1");
    
    rec.push_back(f0);
    rec.push_back(f1);
    rec.push_back(f2);
    rec.push_back(f3);
    rec.push_back(f4);

    assert(rec.length() == 50);
    assert(rec.size() == 5);

    assert(rec.get_field_value("FIELD_0") == "AAAAAAAAAA");

    for (int i=0; i<=4; i++)
    {
        assert(rec[i].name() == "FIELD_"+to_string(i));
        assert(rec[i].description() == "Field desc "+to_string(i));
        assert(rec[i].type().name() == to_string(i));
        assert(rec[i].index() == i);

        assert(rec.contains("FIELD_"+to_string(i)));
    }
    assert(!rec.contains("FOO"));

    // const & non-const iterator
    int i=0;
    for (auto &f: rec) 
    {
        assert(rec[i].name() == "FIELD_"+to_string(i));
        assert(rec[i].description() == "Field desc "+to_string(i));
        assert(rec[i].type().name() == to_string(i));
        assert(rec[i].index() == i);

        assert(rec.contains("FIELD_"+to_string(i)));

        i++;
    }

    i=0;
    for (const auto &f: rec) 
    {
        assert(rec[i].name() == "FIELD_"+to_string(i));
        assert(rec[i].description() == "Field desc "+to_string(i));
        assert(rec[i].type().name() == to_string(i));
        assert(rec[i].index() == i);

        assert(rec.contains("FIELD_"+to_string(i)));

        i++;
    }
}

void test_layout()
{
    //auto layout= Layout(xmlfile);
    Layout layout{xmlfile};

    assert(layout.contains("CONT"));
    assert(!layout.contains("FOO"));

    auto& cont = layout["CONT"];
    auto& coun = layout["COUN"];

    assert(cont->name() == "CONT");
    assert(coun->name() == "COUN");

    assert((*cont)[0].description() == "Record ID");
    assert((*cont)[1].description() == "Name of the continent");

}

void test_reader()
{
    //auto layout= Layout(xmlfile);
    Layout layout{xmlfile};
    Reader reader(rbffile, layout, [](string s) { return s.substr(0,4); });

    for (auto &rec: reader)
    {
        cerr << rec->value(';') << endl;
    }
}
