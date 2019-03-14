#ifndef RECORD_H
#define RECORD_H

#include <string>

using namespace std;

#include <vector>
#include <unordered_map>
#include <memory>
#include <iterator>
#include <sstream>

#include <field.h>

namespace rbf
{

    constexpr int RECORD_INITIAL_SIZE = 100;

    /*!
     * @class Record
     * @brief This class defines a generic record made of fields.
     * @details It allows easy construction and manipulation of a record, which
     * is made of fields.
     *
     * **Example**
     *
     * @code
     *  auto ft1 = FieldType("A/N", "string");
     *  auto f1 = Field("FIELD_A", "Field desc 1", ft1, 15);
     *  auto f2 = Field("FIELD_B", "Field desc 2.1", FieldType("A", "string"), 10);
     *  auto f3 = Field("FIELD_C", "Field desc 3", ft1, 5);
     *  auto f4 = Field("FIELD_B", "Field desc 2.2", FieldType("N", "decimal"), 10);
     *  auto f5 = Field("FIELD_B", "Field desc 2.3", ft1, 10);
     *
     *  auto rec = Record("RECORD1", "Desc for record 1");
     *  rec.push_back(f1);
     *  rec.push_back(f2);
     *  rec.push_back(f3);
     *  rec.push_back(f4);
     *  rec.push_back(f5);
     *
     *  assert(rec.name() == "RECORD1");
     *  assert(rec.description() == "Desc for record 1");
     *
     *  assert(rec.length() == 50);
     *
     *  assert(rec[1].name() == "FIELD_B");
     *  assert(rec[4].name() == "FIELD_B");
     *  assert(rec[3].type().name() == "N");
     *
     *  assert(rec[0] == f1);
     *
     *  assert(rec.size() == 5);
     *
     *  assert(rec[0].index() == 0);
     *  assert(rec[1].index() == 1);
     *  assert(rec[2].index() == 2);
     *  assert(rec[3].index() == 3);
     *  assert(rec[4].index() == 4);
     *
     *  rec[0].setValue("ABCD");
     *  rec[1].setValue("EFG");
     *  rec[2].setValue("H");
     *  rec[3].setValue("XXXX");
     *  assert(rec.value() == "ABCDEFGHXXXX");
     *
     *  auto s1 = "AAAAAAAAAAAAAAABBBBBBBBBBCCCCCDDDDDDDDDDEEEEEEEEEE";
     *  rec.setValue(s1);
     *  cout << rec[0] << endl;
     *  cout << rec[1] << endl;
     *  assert(rec[0].value() == "AAAAAAAAAAAAAAA");
     *  assert(rec[1].value() == "BBBBBBBBBB");
     *  assert(rec[2].value() == "CCCCC");
     *  assert(rec[3].value() == "DDDDDDDDDD");
     *  assert(rec[4].value() == "EEEEEEEEEE");
     *
     *  auto s2 = "AA";
     *  rec.setValue(s2);
     *  assert(rec[0].value() == "AA");
     *  assert(rec[1].value() == "");
     *  assert(rec[2].value() == "");
     *  assert(rec[3].value() == "");
     *  assert(rec[4].value() == "");
     *
     *  s1 = "AAAAAAAAAAAAAAABBBBBBBBBBCCCCCDDDDDDDDDDEEEEEEEEEEFFFFFFFFFFFFFFFFFFFF";
     *  rec.setValue(s1);
     *  assert(rec[0].value() == "AAAAAAAAAAAAAAA");
     *  assert(rec[1].value() == "BBBBBBBBBB");
     *  assert(rec[2].value() == "CCCCC");
    *  assert(rec[3].value() == "DDDDDDDDDD");
    *  assert(rec[4].value() == "EEEEEEEEEE");
    *
        *  // const & non-const iterator
        *  for (auto const &f: rec) 
        *  {
            *      cout << f << endl;
            *  }
    *  for (auto &f: rec) 
        *  {
            *      cout << f << endl;
            *  } 
    *  for (auto f: rec) 
        *  {
            *      cout << f << endl;
            *  } 
    *  @endcode
        */
        class Record : public DataElement
        {
            private:
                // true if field_name exist

                using FieldList = vector<Field>;
                using FieldPtr = unique_ptr<Field>;

                FieldList _field_list;                            // hold the list of fields
                unordered_map<string, vector<size_t>> _field_map; // hold hashmap of field index having the same name

            public:
                /*!
                 * @brief Record default constructor
                 * @details Create a new **Record** object with empty name and description
                 */
                //Record(): Record("", "") {}
                //Record() = default;
                Record() = delete;

                /*!
                 * @brief Record default constructor
                 * @param[in] name record name
                 * @param[in] description record representation
                 */
                Record(const string& name, const string& description) : DataElement(name, description, 0) 
                {
                    _field_list.reserve(RECORD_INITIAL_SIZE);
                } 

                // copy ctor
                //Record(const Record& r): Record(r._name, r._description) {}
                /*!
                 * @brief Record copy constructor
                 * @details Copy all record metadata and all underlying fields
                 */
                Record(const Record& r);

                // dtor
                virtual ~Record();
                //
                // accessors
                /*!
                 * @return the number of fields in the record
                 */
                inline auto size() const { return _field_list.size(); }

                /*!
                 * @return the string value which is the concatenation of all fields values
                 */
                string value(const char separator = ';') const;

                /*!
                 * @return the string value which is the concatenation of all fields raw_value
                 */
                string raw_value() const;

                /*!
                 * @details set record value by setting all included field values individually
                 * @param[in] string value to set
                 */
                void setValue(string s);

                /*!
                 * @details append a Field object in the record
                 * @param[in] Field object reference
                 */
                void push_back(const Field& f);

                /*!
                 * @details access to a Field object with its index within the record
                 * @param[in] Field index
                 */
                Field& operator[](size_t i);

                /*!
                 * @details access to a Field objects matching argument name
                 * @param[in] field name to fetch
                 * @return an array of Field objects
                 * @warning as a record can include field names having the same name, the return
                 * value is an array
                 */
                vector<Field> operator[](const string& field_name);

                /*!
                 * @details access to a Field value
                 * @param[in] field name to fetch
                 * @return field value
                 * @warning this method returns the value of the first field matching the argument
                 */
                string get_field_value(const string& field_name);
                //string operator()(const string& field_name) const { return operator()(field_name); };

                //string operator()(const char *field_name) { return this->operator()(string(field_name)); }

                /*!
                 * @details iterator to loop through fields
                 */
                vector<Field>::iterator begin() { return _field_list.begin(); }
                vector<Field>::iterator end() { return _field_list.end(); }
                vector<Field>::const_iterator begin() const { return _field_list.begin(); }
                vector<Field>::const_iterator end() const { return _field_list.end(); }
                //vector<Field>::const_iterator cbegin() const { cout << "const_iterator2 called!!" << endl;return _field_list.cbegin(); }
                //vector<Field>::const_iterator cend() const { return _field_list.cend(); }

                /*!
                 * @details test if a field is found in record
                 * @param[in] field name to check existence for
                 * @return true if found
                 */
                bool contains(const string& field_name) const { return _field_map.find(field_name) != _field_map.end(); }

                /*!
                 * @details pre-allocate vector size
                 * @param[in] initial vector size
                 */
                void reserve(size_t initial_size) {  _field_list.reserve(initial_size); }

                /*!
                 * @details remove all fields matching field name
                 * @param[in] field name to remove
                 * @param[in] whether or not re-indexing of fields is done
                 */
                void remove(const string& field_name, bool re_indexing=false);

                // public methods other than standard ones
                friend ostream &operator<<(ostream &output, Record& r);
        };

}

#endif // RECORD_H
