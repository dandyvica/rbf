#include <iostream>
#include <layout.h>

namespace rbf
{

    Layout::Layout(string xml_file, size_t initial_size)
    {
        // save file name for future use
        _xml_file = xml_file;

        // open document
        xml_document doc;
        auto result = doc.load_file(_xml_file.data());

        // test if loading is successful
        if (!result)
        {
            throw runtime_error("unable to open file!");
        }

        // root node is rbfile
        auto root = doc.child("rbfile");

        // lookup for field types
        unordered_map<string, FieldType> ftype_map;

        for (auto node: root.children("fieldtype"))
        {
            string data_representation(node.attribute("name").value());
            string data_description(node.attribute("type").value());

            // add field type in our map to later refer to them
            ftype_map.insert(pair<string, FieldType>(data_representation, FieldType(data_representation, data_description)));
        }

        // now look up records
        for (auto node: root.children("record"))
        {
            string rec_name(node.attribute("name").value());
            string rec_desc(node.attribute("description").value());

            // create record and add it to our map
            auto p_rec = make_unique<Record>(rec_name, rec_desc);
            _record_map.insert(pair<string, RecordPtr>(rec_name, move(p_rec)));

            // pre-allocate record size
            _record_map[rec_name]->reserve(initial_size);

            // look up fields and add them to record
            for (auto field: node.children("field"))
            {
                string field_name(field.attribute("name").value());
                string field_desc(field.attribute("description").value());
                string field_type(field.attribute("type").value());
                string field_length(field.attribute("length").value());

                // get saved FieldType object
                auto ft = ftype_map[field_type];

                // add Field to last record created
                auto f = Field(field_name, field_desc, ftype_map[field_type], stoul(field_length));
                _record_map[rec_name]->push_back(f);
            }
        }

    }
}
