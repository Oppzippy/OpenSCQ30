#!/usr/bin/env python3
import sys
import re

inbound_regex = re.compile(r"^[A-Z]+:\w+:<-- (\[[0-9, ]+\])")
while line := sys.stdin.readline():
    if match := inbound_regex.match(line):
        print(match.group(1))
