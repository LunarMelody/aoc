import os
import io


def read_input(current_file: str):
    result = io.StringIO()
    path = os.path.join(os.path.dirname(current_file), "input.txt")

    with open(path, encoding="utf-8") as fp:
        result.write(fp.read())

    return result.getvalue()
