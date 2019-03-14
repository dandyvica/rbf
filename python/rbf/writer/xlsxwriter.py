import os
import time
import zipfile
import shutil

from rbf.record import Record


class XlsxWriter(object):
    """

    defines a writer for creating an Excel XLSX file from records

    :param output: excel file name
    :type output: str

     :example:
    ::

        xlsxwriter = XlsxWriter("myfile.xlsx")


    """
    def __init__(self, output):
        self._xls_filename = os.path.basename(output)

        # need to hold a list of file handles, one file per worksheet
        self._worksheet_handles = {}

        # create XLSX directory structure
        self._xls_dir = "./" + os.path.basename(output) + "." + str( time.time())

        # check for existence (should not occur!)
        if not os.path.exists(self._xls_dir):
            os.makedirs(self._xls_dir)
            os.makedirs(self._xls_dir + "/_rels")


    def write(self, record):
        """ insert a record as another row in the corresponding worksheet """

        worksheet_filename = self._xls_dir + "/" + record.name + ".xml"

        # worksheet is not yet existing. So create file and hold it into our dict
        if record.name not in self._worksheet_handles:
            self._worksheet_handles[record.name] = open(worksheet_filename, "w")
            self._worksheet_handles[record.name].write("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<sheetData>""")
            # write worksheet column name
            self._worksheet_handles[record.name].write("<row>")
            for f in record:
                self._worksheet_handles[record.name].write('<c t="inlineStr"><is><t>{0}</t></is></c>\n'.format(f.name))
            self._worksheet_handles[record.name].write("</row>")

            # write worksheet column description
            self._worksheet_handles[record.name].write("<row>")
            for f in record:
                self._worksheet_handles[record.name].write('<c t="inlineStr"><is><t>{0}</t></is></c>\n'.format(f.description))
            self._worksheet_handles[record.name].write("</row>")

        # otherwise, start writing cells
        self._worksheet_handles[record.name].write("<row>")
        for f in record:
            if f.field_type in ("N", "I", "D"):
                self._worksheet_handles[record.name].write("<c><v>{0}</v></c>\n".format(f.value))
            else:
                self._worksheet_handles[record.name].write('<c t="inlineStr"><is><t>{0}</t></is></c>\n'.format(f.value))
        self._worksheet_handles[record.name].write("</row>")

    def _create_content_types(self, content_type_tags):
        fh = open(self._xls_dir + "/[Content_Types].xml", "w")
        fh.write("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Override PartName="/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
""")
        fh.write(content_type_tags)
        fh.write("</Types>")
        fh.close()

    def _create_workbook(self, workbook_tags):
        fh = open(self._xls_dir + "/workbook.xml", "w")
        fh.write("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<sheets>\n""")
        fh.write(workbook_tags)
        fh.write("\n</sheets>\n</workbook>")
        fh.close()

    def _create_rels(self):
        fh = open(self._xls_dir + "/_rels/.rels", "w")
        fh.write("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="workbook.xml"/>""")
        fh.write("</Relationships>")
        fh.close()

    def _create_workbook_rels(self, worksheet_rel_tags):
        fh = open(self._xls_dir + "/_rels/workbook.xml.rels", "w")
        fh.write("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">""")
        fh.write(worksheet_rel_tags)
        fh.write("</Relationships>")
        fh.close()

    def close(self):
        """ finally, close all opened files and create zip file (.xlsx) """

        worksheet_names = sorted(list(self._worksheet_handles.keys()))

        content_type_tags = worksheet_rel_tags = workbook_tags = ""

        for i,worksheet_name in enumerate(worksheet_names):
            workbook_tags += '<sheet name="{0}" sheetId="{1}" r:id="rId{1}"/>\n'.format(worksheet_name, i+1)
            worksheet_rel_tags += '<Relationship Id="rId{0}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="{1}.xml"/>\n'.format(i+1, worksheet_name)
            #rel_tags += '<Relationship Id="rId{0}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="{1}.xml"/>\n'.format(i+1, worksheet_name)
            content_type_tags +=  '<Override PartName="/{0}.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>\n'.format(worksheet_name)

        self._create_workbook(workbook_tags)
        self._create_rels()
        self._create_workbook_rels(worksheet_rel_tags)
        self._create_content_types(content_type_tags)

        # at the end, complete worksheet XML files
        for worksheet_name, worksheet_handle in self._worksheet_handles.items():
            worksheet_handle.write("</sheetData></worksheet>")
            worksheet_handle.close()

        # now finally, create zip file as an .XLSX file

        # cd to the subdir where files are
        os.chdir(self._xls_dir)

        # build zip file
        with zipfile.ZipFile("../" + self._xls_filename, 'w') as myzip:
            # add worksheets to zip file
            for worksheet_name in worksheet_names:
                myzip.write(worksheet_name + ".xml")

            # add static files
            myzip.write("[Content_Types].xml")
            myzip.write("workbook.xml")
            myzip.write("_rels/.rels")
            myzip.write("_rels/workbook.xml.rels")

        # go back to previous directory
        os.chdir("..")

        # delete subdirectory
        shutil.rmtree(self._xls_dir)




