#!/usr/bin/env python3
# Copyright 2021 Canonical Ltd.
# See LICENSE file for licensing details.
#
# Learn more at: https://juju.is/docs/sdk

import logging

from ops.charm import CharmBase
from ops.main import main
from ops.model import ActiveStatus

logger = logging.getLogger(__name__)


class ProviderCharm(CharmBase):
    """Charm the service."""

    def __init__(self, *args):
        super().__init__(*args)
        self.framework.observe(self.on.workload_pebble_ready, self._on_workload_pebble_ready)
        self.framework.observe(self.on["some-regular-relation"].relation_departed, self._on_relation_departed)

    def _on_workload_pebble_ready(self, event):
        """Define and start a workload using the Pebble API.

        Learn more about Pebble layers at https://github.com/canonical/pebble
        """
        self.unit.status = ActiveStatus()

    def _on_relation_departed(self, event):
        if self.unit.is_leader():
            logger.info("Leader attempts to read remote app relation data...")
            value = event.relation.data.get(event.relation.app).get("requirer-key")
            logger.info("Leader read remote app relation data")

            logger.info("Leader attempts to write its own relation app data...")
            event.relation.data[self.model.app]["provider-key"] = "value"
            logger.info("Leader wrote its own relation data")

        else:
            logger.info("Non-leader attempts to read remote app relation data...")
            value = event.relation.data.get(event.relation.app).get("key")
            logger.info("Non-leader read remote app relation data")

if __name__ == "__main__":
    main(ProviderCharm)
