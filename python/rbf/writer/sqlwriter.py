import os
import sqlite3

#from rbf.config import settings


class SqlWriter:
    """

    defines a writer for inserting data into an sqlite3 db

    :param output: database file name
    :type output: str

    :param tag: optional tag to add as the first column of each table (usually=input file name)

    :example:
    ::

        sqlwriter = SqlWriter("myfile.db")


    """
    def __init__(self, output):

        # db name is actually the output file
        database_name = output

        # connect to database (and auto-create it if not existing)
        self._conn = sqlite3.connect(database_name)
        self._cursor = self._conn.cursor()

        # no tag
        self._tag = ""

    @property
    def tag(self):
        """
        :getter: get tag data
        :setter: set tag data
        :rtype: str

        """
        return self._tag

    @tag.setter
    def tag(self, tag):
        """ set the tag data """
        self._tag = tag

    def commit(self):
        """

        commit data to the db

        """
        self._conn.commit()

    def write(self, rec):
        """

        write a record as a SQL row

        :param rec: record object to insert
        :type output: Record object

        :example:
        ::

            writer.write(rec)


        """
        # build the list of tuples (just 1 here) to insert
        data = [(self._tag,) + tuple(f.value for f in rec)]
        placeholder = ",".join(['?']*len(data[0]))

        table_name = self._build_table_name(rec.name)

        try:
            self._conn.executemany("insert into {0} values ({1})".format(table_name, placeholder), data)

        except sqlite3.Error as e:
            print("inserting data in table RECORD: ", e.args[0])

    def create_schema_from_structure(self, format):
        """
        create structure from the record-based format

        :param format: list of records
        :type format: list

        """
        # build sql orders
        # for each record found in the format, create the corresponding table and fields
        try:
            for recname, rec in format.items():
                copied_record = rec.copy()
                copied_record.autorename()

                # build SQL
                clauses = []
                for field in copied_record:
                    if field.field_type == "N":
                            clause = "{0} REAL"
                    elif field.field_type == "I":
                            clause = "{0} INTEGER"
                    else:
                            clause = "{0} TEXT"

                    clauses.append(clause.format(field.name))

                # build sql order
                sql = ",".join(clauses)
                table_name = self._build_table_name(rec.name)
                self._cursor.execute("create table {0} ({1}, {2});".format(table_name, "TAG TEXT", sql))

                # log
                #settings.logger.info("creating table {0}".format(table_name))

        except sqlite3.Error as e:
            print("error creating table, probably existing: ", e.args[0])

    def close(self):
        """
        close is in fact commit data to db
        """
        self._conn.commit()

    def _build_table_name(self, record_name):
        return "REC"+record_name if record_name.isdigit() else record_name


