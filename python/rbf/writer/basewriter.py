import io

class Writer(object):
    def __init__(self, output: str):
        # depending on output type (file or string), create file handler or IO object
        if output == "":
            self._fh = io.StringIO()
        else:
            self._fh = open(output, "w")