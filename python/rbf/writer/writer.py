"""

This is an object factory for creating specific writer depending
on what is pass as argument

"""
from enum import Enum

from rbf.writer.sqlwriter import SqlWriter
from rbf.writer.textwriter import TextWriter
from rbf.writer.htmlwriter import HtmlWriter
from rbf.writer.xlsxwriter import XlsxWriter

"""
Enum class to define writer style
"""
class WriterStyle(Enum):
    text = 1
    html = 2
    sqlite3 = 3
    xlsx = 4


def writer(output="", style=WriterStyle.text):
    """

    return a specific writer object depending on style

    :param output: output file name
    :type output: str
    :param style: either "text", "html", "sql"
    :type output: str

    :example:
    ::

        writer = writer("myfile.html", style="html")
        writer.write(rec)

    """
    # call appropriate writer
    if style == WriterStyle.html:
        return HtmlWriter(output)
    if style == WriterStyle.text:
        return TextWriter(output)
    if style == WriterStyle.sqlite3:
        return SqlWriter(output)
    if style == WriterStyle.xlsx:
        return XlsxWriter(output)

    # non valid style
    raise ValueError("Unrecognized style {0}".format(style))

