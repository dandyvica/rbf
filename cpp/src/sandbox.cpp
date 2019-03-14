#include <iostream>
#include <cassert>
#include <vector>
#include <map>
#include <memory>
#include <string>

#include <pugixml.hpp>

using namespace std;
using namespace pugi;

#include <element.h>
#include <fieldtype.h>
#include <field.h>
#include <record.h>
#include <layout.h>
#include <reader.h>

void fill_rec(vector<unique_ptr<Field>>& v);
void fill_rec1(vector<Field *>& v);

typedef unique_ptr<Field> FieldPtr;

int main()
{
    vector<FieldPtr> v;
    map<string, FieldPtr>h;

    fill_rec(v);
    cout << v.size() << endl;

    for (auto& f: v)
    {
        cout << f->description() << " " << f->type().name() << endl;
    }


    /*
    vector<Field *> v1;
    fill_rec1(v1);
    cout << v1.size() << endl;
    cout << v1[0]->description() << endl;
    cout << v1[0]->description() << endl;
    */



}

void fill_rec(vector<FieldPtr>& v)
{
    auto ft1 = FieldType("A/N", "string");
   
    for (int i=0; i<=9; i++) 
    {
        v.push_back(make_unique<Field>("FIELD_"+to_string(i), "Field desc "+to_string(i), ft1, 15));
    }
}

void fill_rec1(vector<Field *>& v)
{
    auto ft1 = FieldType("A/N", "string");

    auto f1 = new Field("FIELD_A", "Field desc 1", ft1, 15); cout << &f1 << endl;

    v.push_back(f1);
}
