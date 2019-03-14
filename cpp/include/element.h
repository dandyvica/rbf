#ifndef ELEMENT_T
#define ELEMENT_T

#include <iostream>
#include <string>
using namespace std;

#include <element.h>

namespace rbf
{

    /*!
     * @class Element
     * @brief A templated class which represents the most atomic piece of information within a
     * record-based file
     * @tparam T type used for element length (usually **T** is an integral type)
     * @details This class will be the base class for representing fields and records
     * This is the base class for defining objects found in **record-based files** (rbf).
     * It can be viewed as an atomic structure, a cinder block upon which 
     * rbf data structures are based.
     *
     * **Example**
     *
     * @code
     * auto e1 = Element("ELEMENT1", "This is element #1", 5);
     * auto e2(e1);
     *
     * assert(e1.name() == "ELEMENT1");
     * assert(e1.description() == "This is element #1");
     * assert(e1.length() == 5);
     *
     * assert(e1 == e2);
     * @endcode
     *
     */
    template <class T>
        class Element
        {
            protected:
                string _name;                   ///< element name
                string _description;            ///< element description
                T _length;                      ///< element whole length

            public:
                /*!
                 * @details Just create an **Element** object with empty name, description and zero-length value.
                 */
                //Element(): _name{""}, _description{""}, _length{0} { cout << "Element default ctor called!" << endl; }
                Element() = default;

                /*!
                 * @brief Element class constructor
                 * @param[in] name nickname of the element
                 * @param[in] description detailed description of what is the element
                 * @param[in] length total length (in bytes) of the element
                 *
                 * @code 
                 * auto e1 = Element("ELEMENT1", "This is element #1", 5);
                 * @endcode
                 */
                Element(const string& name, const string& description, const T& length): _name{name}, _description{description}, _length{length} {}

                /*!
                 * @details Copy constructor
                 * @code 
                 * auto e1 = Element("ELEMENT1", "This is element #1", 5);
                 * auto e2(e1);
                 * @endcode
                 */
                //Element(const Element& e) : _name{e._name}, _description{e._description}, _length{e._length} { cout << "Element copy ctor called!" << endl;}
                Element(const Element& e) = default;

                /*!
                 * @details Assignment constructor
                 *
                 * @code 
                 * auto e1 = Element("ELEMENT1", "This is element #1", 5);
                 * auto e2 = e1;
                 * @endcode
                 */
                /*
                   Element& operator=(const Element& e) {
                // test self-assignment
                if (&e == this) return *this;

                // reuse copy ctor
                return this(e);
                }*/
                Element& operator=(const Element& e) = default;

                // getters
                /*!
                 * @details **name** attribute getter
                 */
                inline string name() const { return _name; }

                /*!
                 * @details **description** attribute getter
                 */
                inline string description() const { return _description; }

                /*!
                 * @details **length** attribute getter
                 */
                inline T length() const { return _length; }

                // setters

                /*!
                 * @details **name** attribute setter
                 */
                inline void setName(const string& name) { _name = name; }

                /*!
                 * @details **description** attribute setter
                 */
                inline void setDescription(const string& description) { _description = description; }

                /*!
                 * @details **length** attribute setter
                 */
                inline void setLength(T length) { _length = length; }

                // overloaded ops
                /*!
                 * @details Two Element objects are equals if **name**, **description** and **length** are equal.
                 */
                inline bool operator==(const Element& e) const {
                    return _name == e._name && _description == e._description && _length == e._length;
                }

                /*!
                 * @details Negation of equality
                 */
                inline bool operator!=(const Element& e) const { return !(*this == e); }

        };

    /*! @relates Element
     * @brief This is a special definition for usual fields & records in rbf files .
     * Use of **size_t** integer type makes it portable.
     * 
     */
    using DataElement = Element<size_t>;

}
#endif
