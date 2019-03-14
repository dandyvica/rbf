#include <iostream>
#include "Field.h"
#include "Record.h"

using namespace std;

int main()
{
    // test Field class
    auto f1 = Field("FIELD_A", "Field desc 1", "A", 15);
    auto f2 = Field("FIELD_B", "Field desc 2.1", "AN", 10);
    auto f3 = Field("FIELD_C", "Field desc 3", "I", 5);
    auto f4 = Field("FIELD_B", "Field desc 2.2", "AN", 10);
    auto f5 = Field("FIELD_B", "Field desc 2.3", "AN", 10);

    f1.setValue("ABCD");
    f2.setValue("EFG");
    f3.setValue("H");
    f4.setValue("XXXX");

    // test Record class
    auto rec = Record("RECORD1", "Desc for record 1");
    rec.push_back(f1);
    rec.push_back(f2);
    rec.push_back(f3);
    rec.push_back(f4);
    rec.push_back(f5);


    cout << "testing [int]\n" << *rec[2];

    cout << "testing [string]" << endl;
    for (auto f: rec["FIELD_B"]) {
        cout << *f;
    }

    cout << "testing (), value=" << rec("FIELD_B") << endl;

    // copy
    auto copied = rec;
    cout << "testing copy cons \n" << copied;
    exit(0);
    copied[0]->setName("toto");
    //cout << copied[0];
    exit(0);
    for (auto f: copied) {
        cout << *f;
    }

    cout << "testing iterating used custom loop" << endl;
    for (auto f: rec) {
        cout << *f;
    }

    cout << "auto_rename test" << endl;
    rec.auto_rename();


    for (auto f: rec) {
        cout << *f;
    }

    return 0;
}
