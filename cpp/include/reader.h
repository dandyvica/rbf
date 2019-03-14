#ifndef READER_H
#define READER_H

#include <iostream>
#include <fstream>
#include <functional>

#include <record.h>
#include <layout.h>

using namespace std;

namespace rbf
{

    // helper for all reader data
    struct ReaderData
    {
        string rb_file;
        Layout& layout;
        function <string (string)> mapper;
        ifstream rbf;
    };


    class ReaderIterator
    {
        private:
            ReaderData& _rdata;
            string _current_line;

        public:
            ReaderIterator(ReaderData& rdata): _rdata{rdata} {}

            bool operator!=(const ReaderIterator& it) const;
            ReaderIterator& operator++();
            RecordPtr& operator*();
    };

    class Reader
    {
        private:
            ReaderData _rdata;

        public:

            Reader(const string& rb_file, Layout& layout, function <string (string)> mapper): 
                _rdata{rb_file, layout, mapper} {}

            Reader() = delete;
            Reader(const Reader& other) = delete;
            Reader& operator=(const Reader& other) = delete;


            // to loop through records within a rb-file
            ReaderIterator begin();
            ReaderIterator end();
    };

}

#endif
