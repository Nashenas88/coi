# Contributing to Coi

Coi welcomes contribution from everyone in the form of suggestions, bug
reports, pull requests, and feedback. This document gives some guidance
if you are thinking of helping us.

Please reach out here in a GitHub issue if we can do anything to help you
contribute.

## Submitting bug reports and feature requests

When reporting a bug or asking for help, please include enough details
so that the people helping you can reproduce the behavior you are seeing.
The default bug report template should assist with that task. For some
tips on how to approach this, read about how to produce a
[Minimal, Complete, and Verifiable example].

[Minimal, Complete, and Verifiable example]: https://stackoverflow.com/help/mcve

When making a feature request, please make it clear what problem you
intend to solve with the feature, any ideas for how Coi could support
solving that problem, any possible alternatives, and any disadvantages.
The default feature request template should assist with that task.

## Running the test suite

We encourage you to check that the test suite passes locally before
submitting a pull request with your changes. If anything does not pass,
typically it will be easier to iterate and fix it locally than waiting
for the CI servers to run tests for you.

## In the [`coi`] directory
```sh
# Test all the example code in Coi documentation
make test
```

The reason for using `make` rather than `cargo` is that we run tests
against all feature combinations of the project. The test suite is small
enough that this can be run very quickly.

[`coi`]: https://github.com/Nashenas88/coi/tree/master/coi
## Conduct

In all Coi-related forums, we follow the [Rust Code of Conduct]. For
escalation or moderation issues please contact Paul (Nashenas88 on github)
instead of the Rust moderation team.

[Rust Code of Conduct]: https://www.rust-lang.org/conduct.html
