import os
import sys

from rbf.field import Field
from rbf.record import Record
#from rbf.config import settings

"""

set of methods used to print out record data as an HTML file

"""


class HtmlWriter:
    """

    create a new HTML writer object

    :param output: text file name for output
    :type output: str

    :example:
    ::

        htmlwriter = HtmlWriter("myfile.html")


    """
    def __init__(self, output):
        self._fh = open(output, "w")
        self._fh.write(r'<html><head><link href="{0}" rel="stylesheet" type="text/css"></head><body>'.format(settings.css))
        #settings.logger.info("creating output file {0}".format(output))

    def write(self, rec, description=False, cssclass="header1"):
        """

        write a record as an HTML table

        :param rec: record object
        :type rec: a Record object
        :param description: True if field description is printed in the HTML table
        :type description: str

        :example:
        ::

            writer.write(rec)


        """
        self._fh.write("<h2>{0} - {1}</h2>".format(rec.name, rec.description))
        self._fh.write("<table>")
        self._fh.write("<tr>" + "".join(["<th class=\""+cssclass+"\">"+f.name+"</th>" for f in rec]) + "</tr>\n")
        if description:
            self._fh.write("<tr>" + "".join(["<td>"+f.description+"</td>" for f in rec]) + "</tr>\n")
        self._fh.write("<tr>" + "".join(["<td><pre>"+f.raw_value+"</pre></td>" for f in rec]) + "</tr>\n")
        self._fh.write("</table><br>")

    def close(self):
        self._fh.write("</body></html>\n")
        self._fh.close()
