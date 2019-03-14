#include <field.h>

namespace rbf
{

    void Field::setValue(string s)
    {
        // copy s as-is
        _raw_value = s;

        // strip blanks from s
        size_t first = s.find_first_not_of(' ');

        // check if found a non-blank char
        if (first == string::npos)
        {
            _str_value = "";
        }
        else
        {
            size_t last = s.find_last_not_of(' ');
            //cout << first << " " << last << endl;
            _str_value = s.substr(first, (last-first+1));
        }
    }

    // public methods other than standard ones
    ostream& operator<<(ostream &output, const Field& f) {
        output 
            // << "adress=<" << &f
            << "field name=<" << f._name
            << ">, description=<" << f._description
            << ">, length=<" << f._length
            << ">, type=<" << f._field_type.name()
            << ">, raw_value=<" << f._raw_value
            << ">, value=<" << f._str_value
            << ">, offset=<" << f._offset
            << ">, lower_bound=<" << f._lower_bound
            << ">, upper_bound=<" << f._upper_bound
            << ">, index=<" << f._index
            << ">, offset=<" << f._offset
            << "> ";
        return output;
    }
}
