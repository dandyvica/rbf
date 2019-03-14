#include <iostream>

#include <record.h>

namespace rbf
{

    Record::~Record()
    {
        // clear data structures
        _field_list.clear();
        _field_map.clear();
    }

    Record::Record(const Record& rec)
    {
        for (auto f: rec)
        {
            this->push_back(f);
        }
    }


    // add a field into a record
    void Record::push_back(const Field& f) 
    {
        // add field to the list
        _field_list.push_back(f);

        // get a reference on last item
        auto &last = _field_list.back();

        // adjust length and index
        last.setIndex(_field_list.size()-1);
        last.setOffset(_length);

        // record length is then larger
        _length += f.length();

        // calculate bounds
        last.setLowerBound(last.offset());
        last.setUpperBound(last.offset() + last.length());

        // autopopulate map is not present
        _field_map[last.name()].push_back(last.index());
    }

    string Record::value(const char separator) const
    {
        stringstream ss;
        for (auto const &f: _field_list) { ss << f.value() << separator; }
        return ss.str();
    }

    string Record::raw_value() const
    {
        /*
        string s;
        for (auto const &f: _field_list) { s += f.raw_value(); }
        return s;
        */
        stringstream ss;
        for (auto const &f: _field_list) { ss << f.raw_value(); }
        return ss.str();
    }

    void Record::setValue(string s)
    {
        // check s length to match record length
        auto len = s.length();
        if (len < _length)
        {
            // left-pad with blanks
            s += string(_length-len, ' ');
        }

        // get the slice of the input string
        for (auto &f: _field_list) 
        { 
            f.setValue(s.substr(f.lower_bound(), f.length())); 
        }
    }

    ostream &operator<<(ostream &output, Record& r)
    {
        output << "record name=<" << r.name() << "> record desc=<" << r.description() << ">" << endl;
        for (auto const &f: r) { output << f << endl; }
        return output;
    }

    vector<Field> Record::operator[](const string& field_name) 
    { 
        vector<Field> flist;
        for (auto i: _field_map[field_name])
        {
            flist.push_back(_field_list[i]);
        }
        return flist;
    }

    // return value
    string Record::get_field_value(const string& field_name) 
    {
        // test if field name is in record
        if (!contains(field_name))
            throw runtime_error("not in record");
        // get index of the first field matching the field name and return its value
        auto index_of_first = _field_map[field_name][0];
        return _field_list[index_of_first].value(); 
    }

    Field& Record::operator[](size_t i) 
    {
        if (i < _field_list.size()) 
        {
            return _field_list.at(i);
        }
        else 
        {
            cerr << "index " << i << " not found in record " << _name << endl;
            abort();
        }
    }

    void Record::remove(const string& field_name, bool re_indexing)
    {
        //for (auto it = _field_map[field_name].end(); it != _field_map[field_name].begin(); --it)
        for (int i =  _field_map[field_name].size()-1; i >= 0; i--)
        {
            cout << field_name << " i=" << _field_map[field_name][i] << endl;
            _field_list.erase(_field_list.begin()+i);
        }
    }

}
