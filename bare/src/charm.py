#!/usr/bin/env python3
# Copyright 2021 Canonical Ltd.
# See LICENSE file for licensing details.
#
# Learn more at: https://juju.is/docs/sdk

import os
import sys
from subprocess import call

if __name__ == "__main__":
    # /var/lib/juju/tools/unit-bare-0/ is already in the PATH, so can call hooks without full path.
    # (os.environ is inherited to the callee.)
    call(["juju-log", "-l", "INFO", str(os.environ)])
    call(["juju-log", "-l", "INFO", str(sys.argv)])
    call(["status-set", "active", "Woohoo!"])
