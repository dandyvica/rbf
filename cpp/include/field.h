#ifndef FIELD_H
#define FIELD_H

#include <iostream>
#include <string>
#include <stdexcept>
#include <sstream>

using namespace std;

#include <element.h>
#include <fieldtype.h>

namespace rbf
{

    /*!
     * @class Field
     * @brief Provide the Field class to manage text fields within a record-based file
     * @details Define a data field with its name, description, type and length
     * This class should be used with its companion class **Record**. If a record can
     * be mapped to a line of text within a file, then a field is a substring from
     * that line, with a fixed length.
     *
     * Each field is holding the substring in the **value** and **raw_value** properties.
     * a record can represent numerical, alphumerical, etc type of data. This class holds the type
     * of field.
     *
     * @todo 
     * * copy at least length-chars when setting the value
     * * type() should return a ref
     *
     * **Example**
     *
     * @code
     *  auto ft1 = FieldType("A/N", "string");
     *  auto f1 = Field("FIELD_1", "Field1 description", ft1, 15);
     *  auto f2(f1);
     *  auto f3 = f2;
     *  
     *  assert(f1.type().name() == "A/N");
     *  f1.setValue("    value1    ");
     *
     *  assert(f1.value() == "value1");
     *  assert(f1.raw_value() == "    value1    ");
     *
     *  assert(f1 == f2);
     *  assert(f2 == f3);
     * @endcode
     */
    class Field : public DataElement
    {
        private:
            FieldType _field_type;    // associated FieldType object

            string _str_value;        // when set, store the value of the field (blank stripped)
            string _raw_value;        // when set, store the raw value of the field

            unsigned int _index {0};  // index of the field within a record
            unsigned int _offset {0}; // offset of the field among its brothers
            unsigned int _lower_bound {0}; // lower & upper bounds when field is added to a record
            unsigned int _upper_bound {0};

        public:
            /*!
             * @brief Field default constructor
             * @details Create a new **Field** object with empty name, description, null length and VOID **type**
             */
            //Field(): DataElement(), _field_type(FieldType()) { cout << "Field default ctor called!" << endl; cout.flush(); }
            Field() = default;

            /*!
             * @brief Field class constructor
             * @param[in] name field name
             * @param[in] description field representation
             * @param[in] type field type object
             * @param[in] length field length
             *
             * @code 
             * auto ft1 = FieldType("ALPHA", "string");
             * auto f1 = Field("FIELD1", "This is field #1", ft1, 10);
             * @endcode
             */
            Field(const string& name, const string& description, const FieldType& type, const size_t& length) : 
                DataElement(name, description, length), _field_type(type) { 
                    _raw_value.reserve(length); 
                    _str_value.reserve(length); 
                } 

            /*!
             * @brief Field class copy constructor
             */
            //Field(const Field& f): Field(f._name, f._description, f._field_type, f._length) { cout << "Field " << f._name << " is copied!!" << endl;}
            Field(const Field& f) = default;

            // accessors & mutators
            /*!
             * @details **value** attribute getter
             * @return the left and right-stripped value of the field
             */
            inline string value() const { return _str_value; }

            /*!
             * @details **raw_value** attribute getter
             * @return the non-modified value of the field (i.e. non-stripped)
             */
            inline string raw_value() const { return _raw_value; }

            /*!
             * @details **type** attribute getter
             * @return the FieldType object
             */
            inline FieldType& type() { return _field_type; }

            /*!
             * @details **index** attribute getter
             * @return the index value (sequence number or like an array index) of the field. This is
             * only meaningful when the field is added into a Record object. Starts at 0.
             */
            inline unsigned int index() const { return _index; }

            /*!
             * @details **offset** attribute getter
             * @return the offset value of the field. This is
             * only meaningful when the field is added into a Record object. Starts at 0.
             */
            inline unsigned int offset() const { return _offset; }

            /*!
             * @details **lower_bound** attribute getter
             * @return the offset (relative to 0) of the first character in the field when added into a Record
             */
            inline unsigned int lower_bound() const { return _lower_bound; }

            /*!
             * @details **upper_bound** attribute getter
             * @return the offset (relative to 0) of the last character in the field when added into a Record
             */
            inline unsigned int upper_bound() const { return _upper_bound; }

            /*!
             * @details set the **value** and **raw_value** attribute from a string
             * @param[in] s string value to store. Only **lenght** characters are saved, and **value** is blank-stripped
             */
            void setValue(string s);

            /*!
             * @details set the **raw_value** attribute (only)
             * @param[in] s string value
             */
            inline void setRawValue(const string& s) { _raw_value = s; }

            /*!
             * @details **index** attribute setter
             */
            inline void setIndex(unsigned int i) { _index = i; }

            /*!
             * @details **offset** attribute setter
             */
            inline void setOffset(unsigned int offset) { _offset = offset; }

            /*!
             * @details **lower_bound** attribute setter
             */
            inline void setLowerBound(unsigned int lb) { _lower_bound = lb; }

            /*!
             * @details **upper_bound** attribute setter
             */
            inline void setUpperBound(unsigned int ub) { _upper_bound = ub; }

            // overloaded ops
            /*!
             * @details Two Field objects are equals if inherited Element class attributes are equal
             * and **data_type** are equal.
             */
            inline bool operator==(const Field& f) const { return DataElement::operator==(f) && _field_type == f._field_type; }

            /*!
             * @details Negation of equality
             */
            inline bool operator!=(const Field& f) const { return !(*this == f); }

            // public methods other than standard ones
            friend ostream &operator<<(ostream &output, const Field& f);

    };

}

#endif // FIELD_H
