#!/usr/bin/env python3

if __name__ == "__main__":
    import os
    import sys
    import warnings

    from inspect import cleandoc

    major = sys.version_info.major
    minor = sys.version_info.minor

    skip = os.environ.get("HARTEX_IGNORE_OLD_PYTHON") == "1"
    if not skip and (major < 3 or (major == 3 and minor < 11)):
        msg = cleandoc(f"""
            Using Python {major}.{minor}, but Python version 3.11 or higher is recommended.
            This Python version should work for the near future, but eventually this will be
            changed.

            This message can be suppressed by setting `HARTEX_IGNORE_OLD_PYTHON=1`.
        """)

        warnings.warn(msg, stacklevel=1)

        pwd = os.path.dirname(os.path.absname(__file__))
        sys.path.insert(0, os.path.join(pwd, "bootstrap"))

        import bootstrap
        bootstrap.main()
