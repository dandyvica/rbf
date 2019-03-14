#include <reader.h>

namespace rbf
{

    RecordPtr& ReaderIterator::operator*()
    {
        // try to match the record from line read from input file
        getline(_rdata.rbf, _current_line); 
        auto recname = _rdata.mapper(_current_line);
        _rdata.layout[recname]->setValue(_current_line);

        return _rdata.layout[recname];
    }

    ReaderIterator& ReaderIterator::operator++()
    {
        getline(_rdata.rbf, _current_line); 
        return *this;
    }

    bool ReaderIterator::operator!=(const ReaderIterator& it) const 
    { 
        return !_rdata.rbf.eof(); 
    }

    ReaderIterator Reader::begin()  
    {
        _rdata.rbf.open(_rdata.rb_file); 
        if (!_rdata.rbf.is_open())
        {
            throw runtime_error("Unable to open file");
        }
        return ReaderIterator(_rdata); 
    }

    ReaderIterator Reader::end()  
    {
        return ReaderIterator(_rdata);
    }

}
