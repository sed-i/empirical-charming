#!/usr/bin/env python3
# Copyright 2021 Canonical Ltd.
# See LICENSE file for licensing details.
#
# Learn more at: https://juju.is/docs/sdk

import logging
from typing import Optional

from ops.charm import CharmBase
from ops.main import main
from ops.pebble import Error as PebbleError

logger = logging.getLogger(__name__)


class MyCharm(CharmBase):

    def __init__(self, *args):
        super().__init__(*args)
        self._container = self.unit.get_container("workload")

        # self.framework.observe(self.on.workload_pebble_ready, self._on_anything)
        # self.framework.observe(self.on.start, self._on_anything)
        self.framework.observe(self.on.config_changed, self._on_anything)

    def _on_anything(self, _):
        if not self._container.can_connect():
            return
        self.attempt_push()

    def attempt_push(self):
        # Push into mounted storage
        path = "/mounted-storage/somewhere/over/the/rainbow.txt"
        content = "Way up high"
        logger.info("Attempting to push to %s", path)
        self._push(path, content)
        assert self._pull(path) == content

        # Push elsewhere
        path = "/somewhere/over/the/rainbow.txt"
        content = "Way up high"
        logger.info("Attempting to push to %s", path)
        self._push(path, content)
        assert self._pull(path) == content

    def _pull(self, path) -> Optional[str]:
        """Pull file from container (without raising pebble errors).

        Returns:
            File contents if exists; None otherwise.
        """
        try:
            return self._container.pull(path, encoding="utf-8").read()
        except (FileNotFoundError, PebbleError):
            return None

    def _push(self, path, contents):
        """Push file to container, creating subdirs as necessary."""
        logger.info("Pushing with make_dirs: %s", path)
        self._container.push(path, contents, make_dirs=True, encoding="utf-8")


if __name__ == "__main__":
    main(MyCharm)
