# SigmaOS-network

Part of the SigmaOS multi-repository architecture.

## WireGuard status

The exported WireGuard tunnel types are scaffolds, not secure VPNs. They do
not implement Curve25519, Noise, or authenticated encryption. Handshake and
packet transmission methods now return an explicit unavailable error, and no
plaintext packet is framed with a fabricated authentication tag. The zeroed
key fields are placeholders and must not be used as credentials.

## License
SPDX-License-Identifier: GPL-3.0-or-later

## Copyright
Copyright (C) 2026 SigmaOS Project
