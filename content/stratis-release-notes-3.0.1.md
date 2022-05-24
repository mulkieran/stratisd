+++
title = "Stratis 3.1.0 Release Notes"
date = 2022-05-24
weight = 24
template = "page.html"
render = true
+++

*mulhern, Stratis Team*

Stratis 3.1.0 includes significant improvements to the management of the 
thin-provisioning layers, as well as a number of other user-visible
enhancements and bug fixes.

Please see [FIXME] for a detailed discussion of the thin-provisioning
changes. To support these changes the Stratis CLI has been enhanced to:
* allow specifying whether or not a pool may be overprovisioned on creation
* allow changing whether or not a pool may be overprovisioned while it is
running 
* allow increasing the filesystem limit for a given pool
* display whether or not a pool is overprovisioned in the pool list view

Users of the Stratis CLI may also observe the following changes:
* A `debug` subcommand has been added to the `pool`, `filesystem`, and
`blockdev` subcommands. Debug commands are not fully supported and may change
or be removed at any time.
* The `--redundancy` option is no longer available when creating a pool. This
option had only one permitted value so specifying it never had any effect. 

stratisd 3.1.0 includes a number of additional user-visible changes: 
* The minimum size of a Stratis filesystem is increased to 512 MiB.

stratisd 3.1.0 also includes a number of internal improvements:
* The size of any newly created MDV is increased to 512 MiB.
* A pool's MDV is mounted in a private mount namespace and remaines mounted
while the pool is in operation.

<!-- more -->

NOTE: `stratisd` depends directly on the `chrono` crate against which
[RUSTSEC-2020-0159] has been filed. We have demonstrated that `stratisd` is
not affected by this CVE by building and testing `stratisd` against a
clone of the `chrono` crate from which all the code affected by the CVE
has been removed, proving that `stratisd` has no dependency on that code.

Please consult the [stratisd] and [stratis-cli] changelogs for additional
information about the 3.1.0 release.

[FIXME]: FIXME
[stratisd]: https://github.com/stratis-storage/stratisd/blob/FIXME.txt
[stratis-cli]: https://github.com/stratis-storage/stratis-cli/blob/FIXME.txt
[RUSTSEC-2020-0159]: https://rustsec.org/advisories/RUSTSEC-2020-0159
