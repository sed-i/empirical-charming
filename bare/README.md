# "bare" empirical charming

## Description

This is a bare charm, that does not use ops (operator framework), and does
nothing but log hook/action names.

## Usage

```shell
charmcraft pack

juju deploy ./bare_ubuntu-20.04-amd64.charm bare1 --num-units 2
juju deploy ./bare_ubuntu-20.04-amd64.charm bare2 --num-units 2

juju relate bare1:some-regular-provider bare2:some-regular-requirer

juju remove-application bare1 bare2
```

## Relations
The charm's metadata declares two relations so that this charm
could be related to itself for testing purposes.

Provides:
- some-regular-provider

Requires:
- some-regular-requirer

## OCI Images

None.

## Contributing

<!-- TEMPLATE-TODO: Change this URL to be the full Github path to CONTRIBUTING.md-->

Please see the [Juju SDK docs](https://juju.is/docs/sdk) for guidelines on enhancements to this
charm following best practice guidelines, and
[CONTRIBUTING.md](https://github.com/<name>/<operator>/blob/main/CONTRIBUTING.md) for developer
guidance.
