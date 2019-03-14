/**
 * @file FieldType defintions
 */
#ifndef FIELDTYPE_H
#define FIELDTYPE_H

#include <element.h>

namespace rbf 
{

    /*!
     * @enum DataType
     * @brief All possible field types for a field in a record-based file.
     * @details As a record can represent numerical, alphumerical, etc type of data. This class holds the type
     * of field.
     */
    enum class DataType 
    {
        DECIMAL,            ///< represent floating numbers
        INTEGER,            ///< represent integer numbers (positive or negative)
        DATE,               ///< represent data values
        STRING,             ///< represent alphanumerical or alphabetical data
        VOID,               ///< no value
    };

    /*!
     * @class FieldType
     * @brief Define a field type (to be used with the Field class)
     * @details Even if a field within a record-based file is nothing but an ASCII file, some fields within
     * a record can represent numerical, alphumerical, etc type of data. This class holds the type
     * of field.
     *
     * **Example**
     *
     * @code
     * auto ft1 = FieldType("NUMERICAL", "decimal");
     *
     * assert(ft1.name() == "NUMERICAL");
     * assert(ft1.description() == "decimal");
     * assert(ft1.data_type() == DataType::DECIMAL);
     * @endcode
     */
    class FieldType : public DataElement
    {
        DataType _data_type;         ///< field type converted to enum type

        public:
        /*!
         * @brief FieldType default constructor
         * @details Create a new **FieldType** object with empty description, representation and VOID **DataType**
         */
        FieldType(): DataElement(), _data_type(DataType::VOID) {}

        /*!
         * @brief FieldType class constructor
         * @param[in] name type nickname
         * @param[in] description type representation
         *
         * @code 
         * auto ft1 = FieldType("NUM", "decimal");
         * @endcode
         */
        FieldType(const string& name, const string& description);

        /*!
         * @brief FieldType class copy constructor
         *
         * @code 
         * auto ft1 = FieldType("NUM", "decimal");
         * auto ft2(ft1);
         * @endcode
         */
        //FieldType(const FieldType& ft): FieldType(ft.name(), ft.description()) {}
        FieldType(const FieldType& ft) = default;

        // getters
        /*!
         * @details **data_type** attribute getter
         */
        inline DataType data_type() const { return _data_type; }

        // overloaded ops
        /*!
         * @details Two FieldType objects are equals if **name**, **description** 
         * and **DataType** are equal.
         */
        inline bool operator==(const FieldType& e) const { return DataElement::operator==(e) && _data_type == e._data_type; }

        /*!
         * @details Negation of equality
         */
        inline bool operator!=(const FieldType& e) const { return !(*this == e); }
    };

}

#endif
