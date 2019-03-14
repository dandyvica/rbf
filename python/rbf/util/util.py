""" Utility functions """

import os
from collections import namedtuple

def process_args(args, mode):
    """

    save arguments into a name tuple as all scripts have the same arguments template

    :param args: argument list as passed from the command line
    :type args: list


    """
    if len(args) > 4:
        raise ValueError("Wrong number of arguments")

    # if an output filename is given, we want it
    if len(args) == 4:
        output_file = os.path.basename(args[3])
    else:
        output_file = os.path.basename(args[2]) + "." + mode

    # now save remaining args
    xml_mode = "xml." + args[1]
    data_file = args[2]

    Args = namedtuple('Args', 'xml_mode input_file output_file')

    return Args(xml_mode, data_file, output_file)

def read_records(file):
    """
    :param file: file to read containing record
    :return: dict with record names as keys
    """
    recs = {}
    for line in open(file):
        line = line.split(':')

        # extract record name
        rec = line[0].strip()

        # build list of fields
        recs[rec] = []

        fields = [f.strip() for f in line[1].split(',')]

        recs[rec].append(fields)


    return recs

