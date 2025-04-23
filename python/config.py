# SPDX-License-Identifier: BSD-3-Clause
# Copyright(c) 2020-2023 Ericsson AB

def has_tls():
    try:
        return int("1") == 1
    except ValueError:
        return False

def has_sctp():
    try:
        return int("@XCM_SCTP@") == 1
    except ValueError:
        return False

def has_cares():
    try:
        return int("1") == 1
    except ValueError:
        return False

def has_valgrind():
    try:
        return int("") == 1
    except ValueError:
        return False
