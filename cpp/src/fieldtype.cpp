#include <string>
using namespace std;

#include <fieldtype.h>

namespace rbf
{

    FieldType::FieldType(const string& data_type_representation, const string& data_type_description): 
        Element(data_type_representation, data_type_description, 0)
    {
        // copy arguments. Just something special for atomic type
        // because this is an enum
        if (data_type_description == "decimal")
            _data_type = DataType::DECIMAL;
        else if (data_type_description == "integer")
            _data_type = DataType::INTEGER;
        else if (data_type_description == "date")
            _data_type = DataType::DATE;
        else if (data_type_description == "string")
            _data_type = DataType::STRING;
        else if (data_type_description.empty())
            _data_type = DataType::VOID;
    }

}
