<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (C) 2020 Casper Meijn <casper@meijn.net>

This work is licensed under the Creative Commons Attribution-ShareAlike 4.0 International License. 
To view a copy of this license, visit http://creativecommons.org/licenses/by-sa/4.0/ or 
  send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

Klok
====
A smartwatch firmware with a focus on showing time.


Project
-------
In this project I want to create a smartwatch firmware for showing the current time. I bought the PineTime smartwatch 
with the intention to write some interesting software as a pastime. The PineTime community is very welcoming for new 
ideas, but I want to focus on a simple one. I want to work in a agile way towards a functional watch.

Important ideas for the project:

- Everything is under an open-source license and work on other projects is upstreamed.
- Code is in a modern, safe programming language (Rust). For now the underlying OS is not, as I think this is currently not feasible.
- Automated testing should prevent the user from receiving a bad firmware.
- Compatibility with other hardware is preferred. The current focus is PineTime, but choices made should allow other hardware as well.

Current state
-------------
For now it is just ideas and a prototype that just show the time (and nothing other that the time).

Name
----
The name "Klok" comes from the Dutch word for clock. It is also a posh word for watch.

Installation
------------
See [installation](docs/installation.md)

Upgrade
-------
See [upgrade](docs/upgrade.md)

Contributions
-------------
Contributions to the project are welcome. Open an [issue](https://gitlab.com/caspermeijn/klok/-/issues) for 
problems or suggestions. If you created some code or documentation, open a 
[merge request](https://gitlab.com/caspermeijn/klok/-/merge_requests).  

Please make sure that the commit message conforms to [Conventional Commits](https://www.conventionalcommits.org/). This
basically means that the commit title starts with `fix:` for bugfixes and starts with `feat:` for new functionality. 
Some other useful prefixes are `build:`, `ci:`, `docs:`, `style:`, `test:`.

Please make sure that the continuous integration succeeds. If it fails, please adjust the code and amend the commit. 
If you don't know what the problem is, ask a question in a comment. 
