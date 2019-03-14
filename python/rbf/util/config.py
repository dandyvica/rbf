"""
Configuration tags for the rbf.ini file.
"""

import os
import json
import logging


class Config:
    """

    class to gather all configuration read from the .ini config file

    the path/name of the file is given by the environment variable RBFINI, otherwise
    taken from the current directory

    :param env: environment variable for searching rbf.ini file
    :type name: string

    """
    def __init__(self, env="RBFCONF"):

        # read configuration file in the current directory if not found in ENV
        if not os.environ.get(env):
            conf_file = "./rbf.json"
        # or given by the RBFCONF variable (or the one given as argument)
        else:
            # test existence of the config file
            conf_file = os.environ[env]

            if not os.path.isfile(conf_file):
                raise ValueError("configuration file <{0}> not found!".format(conf_file))

        # read whole JSON configuration file
        self.config = json.load(open(conf_file))

        # create logger
        self.logfile = self.config["log"]["logfile"]
        self.loglevel = self.config["log"]["loglevel"]

        self.logger = logging.getLogger("RBF")
        hdlr = logging.FileHandler(self.logfile)
        formatter = logging.Formatter("%(asctime)s-%(name)s-%(levelname)s-%(message)s")
        hdlr.setFormatter(formatter)
        self.logger.addHandler(hdlr)
        self.logger.setLevel(eval(self.loglevel))

        # read mode (whether we stop when reading a shorter or longer line than expected
        self.read_mode = self.config["global"]["read_mode"]
        if self.read_mode != "strict" and self.read_mode != "lazy":
            raise ValueError("only strict or lazy modes !!")

        # path to css
        self.css = self.config["html"]["css"]

# create main settings object
settings = Config()
