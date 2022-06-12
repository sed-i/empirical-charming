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
    if hook_name := os.environ.get("JUJU_HOOK_NAME"):
        call(["juju-log", "-l", "INFO", f"Hook: {hook_name}"])
    else:
        call(["juju-log", "-l", "ERROR", "This is odd: JUJU_HOOK_NAME is not set!"])
    call(["status-set", "active", "Woohoo!"])
