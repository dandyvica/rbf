#ifndef LAYOUT_H
#define LAYOUT_H

#include <map>

#include <element.h>
#include <field.h>
#include <record.h>

#include <pugixml.hpp>

using namespace pugi;
using namespace std;

namespace rbf
{
    /// useful helper
    using RecordPtr = unique_ptr<Record>;


    /// initial record number of fields
    constexpr size_t RECORD_SIZE_INIT = 100;

    /*!
     * @class Layout
     * @brief This class defines a generic record made of fields.
     * @details Read XML file description and load description into records and fields
     *
     * **Example**
     *
     * @code
     *  auto layout= Layout(xmlfile);

     *  assert(layout.contains("CONT"));
     *  assert(!layout.contains("FOO"));

     *  for (auto const &kv: layout)
     *  {
     *      cerr << kv.first <<  " " << kv.second.description() << endl;
     *  }
     *  @endcode
     */
    class Layout
    {
        private:
            string _xml_file;                       // xml file name for underlying layout
            map<string, RecordPtr> _record_map;     // hold records as a map with key = record name

        public:
            /*!
             * @brief Layout deleted constructor
             */
            Layout() = delete;
            Layout(const Layout& other) = delete;
            Layout& operator=(const Layout& other) = delete;

            /*!
             * @brief Layout constructor
             * @param[in] xml_file xml layout file name
             * @param[in] initial_record_size pre_allocate every record in the layout with
             * this parameter
             */
            Layout(string xml_file, size_t initial_record_size = RECORD_SIZE_INIT);

            /*!
             * @brief Record access
             * @param[in] recname record name to get
             * @returns a Record reference the matching record name
             */
            RecordPtr& operator[](string recname) { return _record_map[recname]; }
            //RecordPtr& operator[](const char *recname) { return _record_map[recname]; }

            // for iterating over a record by fields
            map<string, RecordPtr>::iterator const begin() { return _record_map.begin(); }
            map<string, RecordPtr>::iterator const end() { return _record_map.end(); }

            /*!
             * @details test if a record is found in layout
             * @param[in] record name to check existence for
             * @return true if found
             */
            bool contains(string record_name) const { return _record_map.find(record_name) != _record_map.end(); }
    };

}

#endif
