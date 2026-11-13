# Changelog
## v1.1.1794603208058

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture dist/action.js

- [e2e][release][negative] rejects an invalid release version during planning — invalid-1794601933159 (#2255)

## E2E test
- Scenario: release.invalid-version — rejects an invalid release version during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Request validation

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, planning rejects the invalid committed version (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#682143:gh43502344341.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43502344341 (attempt 1)

- Sync csm-ci e2e fixture .release-version

- [e2e][release][negative] rejects a pull request merged by the owner instead of the Server App — 1.0.1794602472935 (#2256)

## E2E test
- Scenario: release.wrong-merger — rejects a pull request merged by the
owner instead of the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. release: The owner intentionally merges the setup PR; release
planning rejects it because the Server App did not merge it (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#9bb93b:gh43503742031.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/43503742031
(attempt 1)

- Sync csm-ci e2e fixture .release-version
## v1.0.1794601238998

- [e2e][release][negative] a tag collision at another merge commit is rejected during planning — 1.0.1794600311560 (#2253)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, release planning rejects a tag pointing to another merge commit (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#a5a308:gh43498137671.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43498137671 (attempt 1)

- Sync csm-ci e2e fixture .release-version

- [e2e][release][negative] rejects a pre-existing release authored outside the Server App — 1.0.1794601238998 (#2254)

## E2E test
- Scenario: release.foreign-release — rejects a pre-existing release authored outside the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge and expected build failure, publication refuses the owner-authored draft release (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#cc19ef:gh43500587320.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43500587320 (attempt 1)
## v1.0.1794600311560

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture dist/action.js

- [e2e][release][positive] Publish the seed release for the tag collision test — 1.0.1794600311560 (#2252)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: seed-release
- Classification: positive
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Server App deliberately merges the seed PR (permitted).
2. release: Publication creates the existing immutable tag used by the next PR (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#a5a308:gh43498137671.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43498137671 (attempt 1)
## v1.0.1794599587888

- Sync csm-ci e2e fixture .release-version

- [e2e][release][negative] a stale committed dist file fails the js-action build and still cleans up the request — 1.0.1794599587888 (#2251)

## E2E test
- Scenario: release.stale-dist — a stale committed dist file fails the js-action build and still cleans up the request
- Case: fixture
- Classification: negative
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful Server App setup merge, stale dist fails the build and publication is skipped (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#30b238:gh43496268750.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43496268750 (attempt 1)
## v1.0.1794598506357

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- [e2e][release][positive] replaying a completed none release preserves the immutable release and asset digests — 1.0.1794598506357 (#2250)

## E2E test
- Scenario: release.none-replay — replaying a completed none release preserves the immutable release and asset digests
- Case: fixture
- Classification: positive
- Checks: Release publication; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves the immutable release and asset digests (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#2349ba:gh43493494621.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43493494621 (attempt 1)
## v1.0.1794597452873

- Sync csm-ci e2e fixture .release-version

- [e2e][release][positive] publishes and replays an immutable terraform-provider release with all seventeen verified assets — 1.0.1794597452873 (#2249)

## E2E test
- Scenario: release.terraform-replay — publishes and replays an immutable terraform-provider release with all seventeen verified assets
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves all seventeen verified immutable assets (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#f9ca50:gh43490701820.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43490701820 (attempt 1)
## v1.0.1794596654542

- Sync csm-ci e2e fixture .release-version

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1794596654542 (#2248)

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#0c2295:gh43488557642.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43488557642 (attempt 1)
## v1.0.1794595985140

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture dist/action.js

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1794595985140 (#2247)

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#d6aaea:gh43486812370.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43486812370 (attempt 1)
## v1.0.1794595325020

- Sync csm-ci e2e fixture .release-version

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1794595325020 (#2246)

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#55a9d5:gh43485063781.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43485063781 (attempt 1)
## v1.0.1794594637202

- Sync csm-ci e2e fixture .release-version

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1794594637202 (#2245)

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#cc6aea:gh43483183236.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43483183236 (attempt 1)
## v1.0.1794593590520

- Sync csm-ci e2e fixture .release-version

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mwf5ixl7-mp09 (#2203)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: approve,merge,policy@1f8af88\#977c0e:gh43456147553.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43456147553 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mwf5j0p3-3krf (#2205)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: approve,merge,policy@1f8af88\#977c0e:gh43456147553.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43456147553 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mwf5iz55-clbg (#2204)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: approve,merge,policy@1f8af88\#977c0e:gh43456147553.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43456147553 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mwf5j5ck-2u5v (#2208)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: approve,merge,policy@1f8af88\#977c0e:gh43456147553.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43456147553 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mwf6xk3h-262c (#2233)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: approve,merge,policy@1f8af88\#977c0e:gh43456147553.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43456147553 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mwf7rsyg-t29c (#2235)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: approve,merge,policy@1f8af88\#977c0e:gh43456147553.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43456147553 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mwf945y0-w33s (#2241)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: merge@1f8af88\#316bc6:gh43471510992.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43471510992 (attempt 1)

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1794593590520 (#2244)

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#3613a5:gh43480369519.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43480369519 (attempt 1)
## v1.0.1794582909992

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture dist/action.js

- exercise inferred release PR version

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mwexrcd0-h1ot (#2162)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Harness SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Invocation: merge@a5a4206\#2e3b24:gh43422972549.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43422972549 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mwexrdvs-h2xk (#2163)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Harness SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Invocation: merge@a5a4206\#2e3b24:gh43422972549.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43422972549 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mwexrfet-s8y2 (#2164)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Harness SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Invocation: merge@a5a4206\#2e3b24:gh43422972549.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43422972549 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mwexrk22-15kg (#2167)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Harness SHA: a5a4206d14143351a7203f47f73fbafeebb3ab6d
- Invocation: merge@a5a4206\#2e3b24:gh43422972549.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43422972549 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mweynjdh-gqzd (#2170)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Harness SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Invocation: merge@2d6b19d\#487a58:gh43426494767.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43426494767 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mweynmkk-ms66 (#2172)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Harness SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Invocation: merge@2d6b19d\#487a58:gh43426494767.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43426494767 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mweynkxm-zlva (#2171)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Harness SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Invocation: merge@2d6b19d\#487a58:gh43426494767.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43426494767 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mweynr3a-5562 (#2175)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Harness SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Invocation: merge@2d6b19d\#487a58:gh43426494767.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43426494767 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mwezkd5h-5usy (#2180)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Harness SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Invocation: merge@2d6b19d\#487a58:gh43426494767.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43426494767 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mwf07qjh-b3x7 (#2182)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Harness SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Invocation: merge@2d6b19d\#487a58:gh43426494767.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43426494767 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mwf1zdnb-xar1 (#2190)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Harness SHA: 2d6b19dfd42349d07112c8eebb4406bcf2f752fd
- Invocation: merge@2d6b19d\#bced67:gh43438369849.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43438369849 (attempt 1)

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1794582909992 (#2191)

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Harness SHA: 1f8af88a16729c9a217e9f04757586ccd105f386
- Invocation: release@1f8af88\#99f65f:gh43450596929.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/43450596929 (attempt 1)
## v1.0.1794193361150

- Sync csm-ci e2e fixture .release-version

- [e2e][release][negative] a stale committed dist file fails the js-action build and still cleans up the request — 1.0.1794193361150 (#2153)

## E2E test
- Scenario: release.stale-dist — a stale committed dist file fails the js-action build and still cleans up the request
- Case: fixture
- Classification: negative
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful Server App setup merge, stale dist fails the build and publication is skipped (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#1561aa:gh42554131659.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42554131659 (attempt 1)
## v1.0.1794192867088

- [e2e][release][negative] rejects an invalid release version during planning — invalid-1794192319357 (#2150)

## E2E test
- Scenario: release.invalid-version — rejects an invalid release version during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Request validation

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, planning rejects the invalid committed version (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#b40ef7:gh42551225241.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42551225241 (attempt 1)

- [e2e][release][negative] rejects a pull request merged by the owner instead of the Server App — 1.0.1794192575392 (#2151)

## E2E test
- Scenario: release.wrong-merger — rejects a pull request merged by the
owner instead of the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. release: The owner intentionally merges the setup PR; release
planning rejects it because the Server App did not merge it (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#b40ef7:gh42551225241.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/42551225241
(attempt 1)

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture dist/action.js

- [e2e][release][positive] replaying a completed none release preserves the immutable release and asset digests — 1.0.1794192867088 (#2152)

## E2E test
- Scenario: release.none-replay — replaying a completed none release preserves the immutable release and asset digests
- Case: fixture
- Classification: positive
- Checks: Release publication; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves the immutable release and asset digests (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#950552:gh42553397395.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42553397395 (attempt 1)
## v1.0.1794191970260

- [e2e][release][negative] a tag collision at another merge commit is rejected during planning — 1.0.1794191436879 (#2148)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, release planning rejects a tag pointing to another merge commit (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#b40ef7:gh42551225241.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42551225241 (attempt 1)

- [e2e][release][negative] rejects a pre-existing release authored outside the Server App — 1.0.1794191970260 (#2149)

## E2E test
- Scenario: release.foreign-release — rejects a pre-existing release authored outside the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge and expected build failure, publication refuses the owner-authored draft release (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#b40ef7:gh42551225241.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42551225241 (attempt 1)
## v1.0.1794191436879

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- [e2e][release][positive] Publish the seed release for the tag collision test — 1.0.1794191436879 (#2147)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: seed-release
- Classification: positive
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Server App deliberately merges the seed PR (permitted).
2. release: Publication creates the existing immutable tag used by the next PR (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#b40ef7:gh42551225241.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42551225241 (attempt 1)
## v1.0.1794190572003

- [e2e][release][positive] publishes and replays an immutable terraform-provider release with all seventeen verified assets — 1.0.1794190572003 (#2146)

## E2E test
- Scenario: release.terraform-replay — publishes and replays an immutable terraform-provider release with all seventeen verified assets
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves all seventeen verified immutable assets (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#4a9291:gh42547488529.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42547488529 (attempt 1)
## v1.0.1794190199287

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1794190199287 (#2145)

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#4a9291:gh42547488529.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42547488529 (attempt 1)
## v1.0.1794189836121

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1794189836121 (#2144)

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#4a9291:gh42547488529.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42547488529 (attempt 1)
## v1.0.1794189532797

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1794189532797 (#2143)

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#4a9291:gh42547488529.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42547488529 (attempt 1)
## v1.0.1794189239411

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1794189239411 (#2142)

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#4a9291:gh42547488529.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42547488529 (attempt 1)
## v1.0.1794188942599

- [e2e][release][negative] rejects an invalid release version during planning — invalid-1794074274636 (#2064)

## E2E test
- Scenario: release.invalid-version — rejects an invalid release version during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Request validation

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, planning rejects the invalid committed version (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)

- [e2e][release][negative] rejects a pull request merged by the owner instead of the Server App — 1.0.1794074548986 (#2065)

## E2E test
- Scenario: release.wrong-merger — rejects a pull request merged by the
owner instead of the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. release: The owner intentionally merges the setup PR; release
planning rejects it because the Server App did not merge it (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625
(attempt 1)

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- exercise inferred release PR version

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw76y2u8-xnel (#2074)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Harness SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Invocation: merge@84548b2\#678271:gh42437034325.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42437034325 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw76y4dk-dfb2 (#2075)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Harness SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Invocation: merge@84548b2\#678271:gh42437034325.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42437034325 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw76y5z2-xdcm (#2076)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Harness SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Invocation: merge@84548b2\#678271:gh42437034325.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42437034325 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw76yaiw-dibh (#2079)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Harness SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Invocation: merge@84548b2\#678271:gh42437034325.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42437034325 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mw78b3gi-5jum (#2084)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Harness SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Invocation: merge@84548b2\#678271:gh42437034325.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42437034325 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released — e2e-mw78ucil-zd0w (#2085)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Harness SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Invocation: merge@84548b2\#678271:gh42437034325.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42437034325 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mw79ekw5-qpu4 (#2086)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Harness SHA: 84548b2fd2b40ff4dd179b83fd4a91e1ddf6b28d
- Invocation: merge@84548b2\#678271:gh42437034325.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42437034325 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw8hmdd5-eubo (#2100)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: approve,merge,policy@2db34b6\#d4915d:gh42537200304.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42537200304 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw8hmewh-2axt (#2101)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: approve,merge,policy@2db34b6\#d4915d:gh42537200304.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42537200304 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw8hmgfk-3qmu (#2102)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: approve,merge,policy@2db34b6\#d4915d:gh42537200304.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42537200304 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw8hml2i-8dcb (#2105)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: approve,merge,policy@2db34b6\#d4915d:gh42537200304.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42537200304 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mw8j7ew1-ognm (#2130)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: approve,merge,policy@2db34b6\#d4915d:gh42537200304.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42537200304 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request resolves without leaving an active reservation — e2e-mw8jr1ln-bbi4 (#2131)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub
auto-merge request resolves without leaving an active reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: a pre-existing GitHub auto-merge request resolves without
leaving an active reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: approve,merge,policy@2db34b6\#d4915d:gh42537200304.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/42537200304
(attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mw8jx9rw-n1p4 (#2132)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: approve,merge,policy@2db34b6\#d4915d:gh42537200304.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42537200304 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mw8ktnwv-f2fp (#2138)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: merge@2db34b6\#713d06:gh42544399612.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42544399612 (attempt 1)

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1794188942599 (#2141)

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Harness SHA: 2db34b6c907ba0f948c961c0696799c4cade723c
- Invocation: release@2db34b6\#4a9291:gh42547488529.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42547488529 (attempt 1)
## v1.0.1794073909882

- [e2e][release][negative] a tag collision at another merge commit is rejected during planning — 1.0.1794073355182 (#2062)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, release planning rejects a tag pointing to another merge commit (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)

- [e2e][release][negative] rejects a pre-existing release authored outside the Server App — 1.0.1794073909882 (#2063)

## E2E test
- Scenario: release.foreign-release — rejects a pre-existing release authored outside the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge and expected build failure, publication refuses the owner-authored draft release (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794073355182

- [e2e][release][positive] Publish the seed release for the tag collision test — 1.0.1794073355182 (#2061)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: seed-release
- Classification: positive
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Server App deliberately merges the seed PR (permitted).
2. release: Publication creates the existing immutable tag used by the next PR (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794073081203

- [e2e][release][negative] a stale committed dist file fails the js-action build and still cleans up the request — 1.0.1794073081203 (#2060)

## E2E test
- Scenario: release.stale-dist — a stale committed dist file fails the js-action build and still cleans up the request
- Case: fixture
- Classification: negative
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful Server App setup merge, stale dist fails the build and publication is skipped (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794072717066

- [e2e][release][positive] replaying a completed none release preserves the immutable release and asset digests — 1.0.1794072717066 (#2059)

## E2E test
- Scenario: release.none-replay — replaying a completed none release preserves the immutable release and asset digests
- Case: fixture
- Classification: positive
- Checks: Release publication; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves the immutable release and asset digests (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794072048453

- [e2e][release][positive] publishes and replays an immutable terraform-provider release with all seventeen verified assets — 1.0.1794072048453 (#2058)

## E2E test
- Scenario: release.terraform-replay — publishes and replays an immutable terraform-provider release with all seventeen verified assets
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves all seventeen verified immutable assets (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794071672074

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1794071672074 (#2057)

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794071312347

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1794071312347 (#2056)

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794070994803

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1794070994803 (#2055)

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794070686941

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1794070686941 (#2054)

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1794070388448

- [e2e][release][negative] a tag collision at another merge commit is rejected during planning — 1.0.1793363478809 (#1719)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, release planning rejects a tag pointing to another merge commit (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture .goreleaser.yaml

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvuyp80q-6exq (#1720)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: ca056361242ead9c270ca0de8e2aad8cf1fbd91b
- Harness SHA: ca056361242ead9c270ca0de8e2aad8cf1fbd91b
- Invocation: merge@ca05636\#30e0f1:gh41057247086.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41057247086 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvviuvll-e680 (#1721)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvviux5a-bbm6 (#1722)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvviv3b4-92bk (#1726)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvviuyok-sbbm (#1723)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mvvjhkfb-ngbj (#1731)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request resolves without leaving an active reservation — e2e-mvvjn0ko-nbae (#1732)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub
auto-merge request resolves without leaving an active reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: a pre-existing GitHub auto-merge request resolves without
leaving an active reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575
(attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvvjspgf-ukb2 (#1733)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mvvkdohc-de8p (#1738)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: merge,repo-settings@6b1e8c7\#58886c:gh41135781575.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41135781575 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvvq44os-q59t (#1747)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvvq4ceb-m0g0 (#1752)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvvq47uq-6vie (#1749)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvvq46au-y97r (#1748)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mvvqq8z7-daw9 (#1757)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request resolves without leaving an active reservation — e2e-mvvqvslu-m3r6 (#1758)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub
auto-merge request resolves without leaving an active reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: a pre-existing GitHub auto-merge request resolves without
leaving an active reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515
(attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvvr1pzx-u28a (#1759)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mvvrl0s2-d9g7 (#1764)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Harness SHA: b18aad95ef110914ff1dd5fe756350035435d81c
- Invocation: merge,repo-settings@b18aad9\#f08617:gh41155567515.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41155567515 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvvsfyvu-56rx (#1769)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Harness SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Invocation: merge@3ff9f67\#d95be6:gh41160599231.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41160599231 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvvsg6nj-e4m8 (#1774)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Harness SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Invocation: merge@3ff9f67\#d95be6:gh41160599231.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41160599231 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvvsg0fd-8set (#1770)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Harness SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Invocation: merge@3ff9f67\#d95be6:gh41160599231.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41160599231 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvvsg209-qn7c (#1771)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Harness SHA: 3ff9f6722cc365813a21621657e231c694d4b500
- Invocation: merge@3ff9f67\#d95be6:gh41160599231.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41160599231 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvvu0nw4-iwho (#1779)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#c20308:gh41163787294.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41163787294 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvvu0r1x-5gmm (#1781)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#c20308:gh41163787294.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41163787294 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvvu0pf3-pwng (#1780)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#c20308:gh41163787294.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41163787294 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvvu0vlt-upny (#1784)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#c20308:gh41163787294.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41163787294 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mvvvdu12-fsl2 (#1789)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#c20308:gh41163787294.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41163787294 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request resolves without leaving an active reservation — e2e-mvvvwy8s-53rq (#1790)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub
auto-merge request resolves without leaving an active reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: a pre-existing GitHub auto-merge request resolves without
leaving an active reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#c20308:gh41163787294.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/41163787294
(attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvvw3aet-raxh (#1791)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#c20308:gh41163787294.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41163787294 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mvvwv8h7-v0su (#1797)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Harness SHA: b18f37f78684a55b7249ae6a21a0f76c01816cf5
- Invocation: merge@b18f37f\#96cd4f:gh41168562533.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41168562533 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw67v62o-uzqc (#1809)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Harness SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Invocation: approve,merge,policy@dbf39eb\#3bb47b:gh42360910716.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42360910716 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw67v7ke-wjp9 (#1810)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Harness SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Invocation: approve,merge,policy@dbf39eb\#3bb47b:gh42360910716.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42360910716 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw67v937-yjzm (#1811)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Harness SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Invocation: approve,merge,policy@dbf39eb\#3bb47b:gh42360910716.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42360910716 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw67vdpn-i6u8 (#1814)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Harness SHA: dbf39eb4e9d8d62ed7a1e42734307d0041d9f320
- Invocation: approve,merge,policy@dbf39eb\#3bb47b:gh42360910716.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42360910716 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw68jvhs-3got (#1846)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Harness SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Invocation: approve,merge,policy@96813e5\#8c2a8e:gh42362230270.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42362230270 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw68jyph-yn4h (#1848)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Harness SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Invocation: approve,merge,policy@96813e5\#8c2a8e:gh42362230270.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42362230270 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw68k378-ztd9 (#1851)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Harness SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Invocation: approve,merge,policy@96813e5\#8c2a8e:gh42362230270.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42362230270 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw68jx19-b7ql (#1847)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Harness SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Invocation: approve,merge,policy@96813e5\#8c2a8e:gh42362230270.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42362230270 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mw69dtwo-u8tu (#1876)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Harness SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Invocation: approve,merge,policy@96813e5\#8c2a8e:gh42362230270.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42362230270 (attempt 1)

- [e2e][merge][positive] csm-ci disables a pre-existing GitHub auto-merge request before merging — e2e-mw69j7qz-ar3n (#1877)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — csm-ci disables a
pre-existing GitHub auto-merge request before merging
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: csm-ci disables a pre-existing GitHub auto-merge request
before merging (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Harness SHA: 96813e57bac18e0901b2b3c7753e7c55d6e234fd
- Invocation: approve,merge,policy@96813e5\#8c2a8e:gh42362230270.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/42362230270
(attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released — e2e-mw6ch4xr-ejtn (#1880)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2dbf95d77a883d3a6c177e4fedc154438264bb53
- Harness SHA: 2dbf95d77a883d3a6c177e4fedc154438264bb53
- Invocation: merge@2dbf95d\#bb84aa:gh42369382816.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42369382816 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw6eesnm-7yma (#1892)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Harness SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Invocation: approve,merge,policy@1888919\#d4bb95:gh42373940868.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42373940868 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw6eeu7l-zeuc (#1893)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Harness SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Invocation: approve,merge,policy@1888919\#d4bb95:gh42373940868.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42373940868 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw6eevr5-ijn3 (#1894)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Harness SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Invocation: approve,merge,policy@1888919\#d4bb95:gh42373940868.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42373940868 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw6ef0e5-pq2h (#1897)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Harness SHA: 18889195aadde10eb3f6811d365762b85cc07ab9
- Invocation: approve,merge,policy@1888919\#d4bb95:gh42373940868.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42373940868 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw6f1cks-foju (#1929)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Harness SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Invocation: approve,merge,policy@be4d16b\#2da15e:gh42375236121.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42375236121 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw6f1e37-k4a8 (#1930)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Harness SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Invocation: approve,merge,policy@be4d16b\#2da15e:gh42375236121.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42375236121 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw6f1kby-i2mv (#1934)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Harness SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Invocation: approve,merge,policy@be4d16b\#2da15e:gh42375236121.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42375236121 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw6f1fp1-blas (#1931)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Harness SHA: be4d16b82fafd6156eb6a246d455f60818fca493
- Invocation: approve,merge,policy@be4d16b\#2da15e:gh42375236121.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42375236121 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw6fjgvd-napm (#1966)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: approve,merge,policy@87221cc\#a35c63:gh42376318070.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42376318070 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw6fjjxp-zbbb (#1968)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: approve,merge,policy@87221cc\#a35c63:gh42376318070.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42376318070 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw6fjiey-sz9z (#1967)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: approve,merge,policy@87221cc\#a35c63:gh42376318070.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42376318070 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw6fjomb-jm52 (#1971)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: approve,merge,policy@87221cc\#a35c63:gh42376318070.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42376318070 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mw6gtfbt-9wyl (#1996)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: approve,merge,policy@87221cc\#a35c63:gh42376318070.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42376318070 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released — e2e-mw6h6ka7-n7k8 (#1997)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: approve,merge,policy@87221cc\#a35c63:gh42376318070.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42376318070 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mw6hmm1l-1om8 (#1998)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: approve,merge,policy@87221cc\#a35c63:gh42376318070.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42376318070 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mw6ilbql-b7df (#2013)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#57c2a3:gh42382963134.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42382963134 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mw6ildba-hnff (#2014)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#57c2a3:gh42382963134.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42382963134 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mw6ileto-lki4 (#2015)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#57c2a3:gh42382963134.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42382963134 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mw6iljgk-lh7z (#2018)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#57c2a3:gh42382963134.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42382963134 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mw6ju0rb-iiau (#2023)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#57c2a3:gh42382963134.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42382963134 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released — e2e-mw6kagt3-5xf9 (#2024)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: a pre-existing GitHub auto-merge request is disarmed before its required-check gate is released (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#57c2a3:gh42382963134.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42382963134 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mw6kswap-07jg (#2025)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#57c2a3:gh42382963134.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42382963134 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mw6ltp0v-h739 (#2031)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: merge@87221cc\#775ca8:gh42389577081.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42389577081 (attempt 1)

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1794070388448 (#2053)

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Harness SHA: 87221cc846b7ef962272ae664b66e001dd584b7d
- Invocation: release@87221cc\#600cbc:gh42393433625.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/42393433625 (attempt 1)
## v1.0.1793363478809

- [e2e][release][positive] Publish the seed release for the tag collision test — 1.0.1793363478809 (#1718)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: seed-release
- Classification: positive
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Server App deliberately merges the seed PR (permitted).
2. release: Publication creates the existing immutable tag used by the next PR (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793363212139

- [e2e][release][negative] a stale committed dist file fails the js-action build and still cleans up the request — 1.0.1793363212139 (#1717)

## E2E test
- Scenario: release.stale-dist — a stale committed dist file fails the js-action build and still cleans up the request
- Case: fixture
- Classification: negative
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful Server App setup merge, stale dist fails the build and publication is skipped (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793362841704

- [e2e][release][positive] replaying a completed none release preserves the immutable release and asset digests — 1.0.1793362841704 (#1716)

## E2E test
- Scenario: release.none-replay — replaying a completed none release preserves the immutable release and asset digests
- Case: fixture
- Classification: positive
- Checks: Release publication; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves the immutable release and asset digests (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793362238576

- [e2e][release][positive] publishes and replays an immutable terraform-provider release with all seventeen verified assets — 1.0.1793362238576 (#1715)

## E2E test
- Scenario: release.terraform-replay — publishes and replays an immutable terraform-provider release with all seventeen verified assets
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves all seventeen verified immutable assets (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793361880087

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1793361880087 (#1714)

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793361532599

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1793361532599 (#1713)

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793361219373

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1793361219373 (#1712)

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793360902518

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1793360902518 (#1711)

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793360608799

- [e2e][release][negative] rejects an invalid release version during planning — invalid-1793175458692 (#1368)

## E2E test
- Scenario: release.invalid-version — rejects an invalid release version during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Request validation

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, planning rejects the invalid committed version (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f

- [e2e][release][negative] rejects a pull request merged by the owner instead of the Server App — 1.0.1793175628238 (#1369)

## E2E test
- Scenario: release.wrong-merger — rejects a pull request merged by the
owner instead of the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. release: The owner intentionally merges the setup PR; release
planning rejects it because the Server App did not merge it (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- [e2e][merge][positive] /merge before /approve enables platform auto-merge and GitHub merges after approval — e2e-mvsh33j3-2nq2 (#1371)

## E2E test
- Scenario: merge.concurrent-approve — /merge before /approve enables platform auto-merge and GitHub merges after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge enables platform auto-merge before approval, and GitHub merges once /approve lands (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: fe65f45d471b6bf5648863efda21ba553071c1fc
- Harness SHA: fe65f45d471b6bf5648863efda21ba553071c1fc
- Invocation: merge@fe65f45\#103045
- Runner: Local invocation merge@fe65f45\#103045

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvto86kh-mbx4 (#1389)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Harness SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Invocation: merge@a367998\#8f303c
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40897187086 (attempt 1)

- [e2e][merge][negative] Closing a pull request ends its reservation without a comment — e2e-mvto8fuw-ypb3 (#1395)

## E2E test
- Scenario: merge.closed — Closing a pull request ends its reservation without a comment
- Case: fixture
- Classification: negative
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Closing a pull request ends its reservation without a comment (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Harness SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Invocation: merge@a367998\#8f303c
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40897187086 (attempt 1)

- [e2e][merge][positive] The merger can cancel an active reservation — e2e-mvto8cs2-vdh6 (#1393)

## E2E test
- Scenario: merge.cancel — The merger can cancel an active reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The merger can cancel an active reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Harness SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Invocation: merge@a367998\#8f303c
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40897187086 (attempt 1)

- [e2e][merge][negative] An untrusted requester cannot cancel another merger reservation — e2e-mvto8ecm-muv7 (#1394)

## E2E test
- Scenario: merge.cancel-unauthorized — An untrusted requester cannot cancel another merger reservation
- Case: fixture
- Classification: negative
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: An untrusted requester cannot cancel another merger reservation (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Harness SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Invocation: merge@a367998\#8f303c
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40897187086 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvto884r-5zrz (#1390)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Harness SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Invocation: merge@a367998\#8f303c
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40897187086 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvto89ok-2jzc (#1391)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Harness SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Invocation: merge@a367998\#8f303c
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40897187086 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvto8hdl-7r7a (#1396)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Harness SHA: a3679982e606c5ff3d9445d61b00f1d4239a59bb
- Invocation: merge@a367998\#8f303c
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40897187086 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvtok7qy-qxiz (#1398)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Harness SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Invocation: merge@5ad813b\#a1900d
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40898765504 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvtokauf-yswp (#1400)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Harness SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Invocation: merge@5ad813b\#a1900d
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40898765504 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvtokik7-8bop (#1405)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Harness SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Invocation: merge@5ad813b\#a1900d
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40898765504 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvtok9co-qyin (#1399)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Harness SHA: 5ad813b3eabdda4eb48bdf7fa261b41759f0acd1
- Invocation: merge@5ad813b\#a1900d
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40898765504 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvtqyiie-fmsq (#1416)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Harness SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Invocation: merge,smoke@b089bea\#05e1d5:gh40910023910.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40910023910 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvtqylow-tymq (#1418)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Harness SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Invocation: merge,smoke@b089bea\#05e1d5:gh40910023910.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40910023910 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvtqyq6a-3hud (#1421)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Harness SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Invocation: merge,smoke@b089bea\#05e1d5:gh40910023910.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40910023910 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvtqyk2k-ckl5 (#1417)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Harness SHA: b089bea363f957dc4ed54476c3ed639551d946c9
- Invocation: merge,smoke@b089bea\#05e1d5:gh40910023910.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40910023910 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvtrggff-4yrd (#1424)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#f98f97:gh40912358040.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40912358040 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvtrgjlw-5l2z (#1426)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#f98f97:gh40912358040.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40912358040 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvtrghy1-moka (#1425)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#f98f97:gh40912358040.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40912358040 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvtromzb-yyi2 (#1431)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#c6f510:gh40913392235.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40913392235 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvtroq58-7x79 (#1433)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#c6f510:gh40913392235.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40913392235 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvtroup3-k5jz (#1436)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#c6f510:gh40913392235.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40913392235 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvtrooim-vcha (#1432)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#c6f510:gh40913392235.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40913392235 (attempt 1)

- [e2e][merge][negative] A reservation waits when its new head has not been approved — e2e-mvtruui0-v7f0 (#1438)

## E2E test
- Scenario: merge.head-change-needs-approval — A reservation waits when its new head has not been approved
- Case: fixture
- Classification: negative
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation waits when its new head has not been approved (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Harness SHA: eaa298673f042cff56bf996734ac511f04943cd0
- Invocation: merge,smoke@eaa2986\#c6f510:gh40913392235.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40913392235 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvtsyoe5-ck9z (#1441)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Harness SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Invocation: merge,smoke@e100a3e\#c38f2a:gh40919191317.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40919191317 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvtsyrij-o999 (#1443)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Harness SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Invocation: merge,smoke@e100a3e\#c38f2a:gh40919191317.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40919191317 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvtsyw4a-mgo1 (#1446)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Harness SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Invocation: merge,smoke@e100a3e\#c38f2a:gh40919191317.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40919191317 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvtsypze-fdnv (#1442)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Harness SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Invocation: merge,smoke@e100a3e\#c38f2a:gh40919191317.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40919191317 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mvttkumj-4rof (#1451)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Harness SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Invocation: merge,smoke@e100a3e\#c38f2a:gh40919191317.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40919191317 (attempt 1)

- [e2e][merge][positive] csm-ci disables a pre-existing GitHub auto-merge request before merging — e2e-mvttq9jp-gmrm (#1452)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — csm-ci disables a pre-existing GitHub auto-merge request before merging
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: csm-ci disables a pre-existing GitHub auto-merge request before merging (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Harness SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Invocation: merge,smoke@e100a3e\#c38f2a:gh40919191317.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40919191317 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvttwbg4-546q (#1453)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Harness SHA: e100a3ec51454ed88a09e80eb518cbfe9a1b58bc
- Invocation: merge,smoke@e100a3e\#c38f2a:gh40919191317.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40919191317 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mvtuyiz9-0an1 (#1461)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 3ab9b9b3bacb1c6c6555c0265f62413400d69e00
- Harness SHA: 3ab9b9b3bacb1c6c6555c0265f62413400d69e00
- Invocation: merge@3ab9b9b\#b3cac4:gh40926673562.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40926673562 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mvtvl35v-xruc (#1465)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 4a9b03b6997d19969ce6ea6e3f9fde2ab01e4235
- Harness SHA: 4a9b03b6997d19969ce6ea6e3f9fde2ab01e4235
- Invocation: merge@4a9b03b\#38b500:gh40929453139.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40929453139 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvuku4jd-556u (#1481)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Harness SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Invocation: approve,merge,policy@06aaec1\#5bf0b8:gh41012777699.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41012777699 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvuku657-77cf (#1482)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Harness SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Invocation: approve,merge,policy@06aaec1\#5bf0b8:gh41012777699.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41012777699 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvuku7oh-jlt4 (#1483)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Harness SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Invocation: approve,merge,policy@06aaec1\#5bf0b8:gh41012777699.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41012777699 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvukuca8-gln2 (#1486)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Harness SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Invocation: approve,merge,policy@06aaec1\#5bf0b8:gh41012777699.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41012777699 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mvulnt5v-14f8 (#1511)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Harness SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Invocation: approve,merge,policy@06aaec1\#5bf0b8:gh41012777699.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41012777699 (attempt 1)

- [e2e][merge][positive] csm-ci disables a pre-existing GitHub auto-merge request before merging — e2e-mvulsy5y-wt03 (#1512)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — csm-ci disables a
pre-existing GitHub auto-merge request before merging
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: csm-ci disables a pre-existing GitHub auto-merge request
before merging (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Harness SHA: 06aaec174af7203d76444b0d02a4475947c6131d
- Invocation: approve,merge,policy@06aaec1\#5bf0b8:gh41012777699.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/41012777699
(attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvum7v47-ajjs (#1524)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Harness SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Invocation: approve,merge,policy@cb1bdf7\#4e237b:gh41016578750.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41016578750 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvum7y5x-gs08 (#1526)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Harness SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Invocation: approve,merge,policy@cb1bdf7\#4e237b:gh41016578750.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41016578750 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvum82t2-a14c (#1529)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Harness SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Invocation: approve,merge,policy@cb1bdf7\#4e237b:gh41016578750.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41016578750 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvum7wmp-ghwx (#1525)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Harness SHA: cb1bdf720d09bec0599abedcb7090942a3b344f2
- Invocation: approve,merge,policy@cb1bdf7\#4e237b:gh41016578750.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41016578750 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvus5a8a-8xlq (#1561)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Harness SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Invocation: approve,merge,policy@31eae85\#2e88a0:gh41034816260.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41034816260 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvus5de1-eajl (#1563)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Harness SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Invocation: approve,merge,policy@31eae85\#2e88a0:gh41034816260.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41034816260 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvus5hyr-5yc6 (#1566)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Harness SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Invocation: approve,merge,policy@31eae85\#2e88a0:gh41034816260.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41034816260 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvus5bsv-gf2s (#1562)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Harness SHA: 31eae85caa268b66aee53e44d8efac99fe3ff8a3
- Invocation: approve,merge,policy@31eae85\#2e88a0:gh41034816260.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41034816260 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvusjgqw-ea68 (#1598)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Harness SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Invocation: approve,merge,policy@bba1792\#1f6265:gh41036154906.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41036154906 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvusjjvj-2047 (#1600)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Harness SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Invocation: approve,merge,policy@bba1792\#1f6265:gh41036154906.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41036154906 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvusjic5-bi7p (#1599)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Harness SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Invocation: approve,merge,policy@bba1792\#1f6265:gh41036154906.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41036154906 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvusjogm-0gne (#1603)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Harness SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Invocation: approve,merge,policy@bba1792\#1f6265:gh41036154906.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41036154906 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mvutd8yz-1s2g (#1628)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Harness SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Invocation: approve,merge,policy@bba1792\#1f6265:gh41036154906.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41036154906 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request resolves without leaving an active reservation — e2e-mvutihxd-3ntz (#1629)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub
auto-merge request resolves without leaving an active reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: a pre-existing GitHub auto-merge request resolves without
leaving an active reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Harness SHA: bba179295946b76f17f8f02cc7fc9efab56980fe
- Invocation: approve,merge,policy@bba1792\#1f6265:gh41036154906.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/41036154906
(attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvutviz1-r07b (#1641)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#bceeb2:gh41040723010.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41040723010 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvuuba5b-9f4f (#1670)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- [e2e][merge][positive] /merge reserves an unapproved pull request and merges it after approval — e2e-mvuubbpu-ba2f (#1671)

## E2E test
- Scenario: merge.reserve-then-approve — /merge reserves an unapproved pull request and merges it after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge creates an active deployment and label, then /approve lets csm-ci merge the current head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- [e2e][merge][positive] A reservation follows a new head and merges after that head is approved — e2e-mvuubd82-i6pu (#1672)

## E2E test
- Scenario: merge.head-change-carries-over — A reservation follows a new head and merges after that head is approved
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A reservation follows a new head and merges after that head is approved (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- [e2e][merge][positive] Editing the original command does not revoke an accepted reservation — e2e-mvuubhty-ark8 (#1675)

## E2E test
- Scenario: merge.command-edit-ignored — Editing the original command does not revoke an accepted reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Editing the original command does not revoke an accepted reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- [e2e][merge][positive] The scheduled sweep evaluates active reservations — e2e-mvuv64ed-2gpc (#1700)

## E2E test
- Scenario: merge.sweep — The scheduled sweep evaluates active reservations
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The scheduled sweep evaluates active reservations (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- [e2e][merge][positive] a pre-existing GitHub auto-merge request resolves without leaving an active reservation — e2e-mvuvc1ed-9cns (#1701)

## E2E test
- Scenario: merge.platform-auto-merge-disabled — a pre-existing GitHub
auto-merge request resolves without leaving an active reservation
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. merge: a pre-existing GitHub auto-merge request resolves without
leaving an active reservation (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner:
https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661
(attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvuvhss2-2unr (#1702)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mvuw15im-wkfq (#1707)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1793360608799 (#1710)

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Harness SHA: 6b1e8c7bc4d168bb6597f8d475f87169563faa8c
- Invocation: approve,merge,policy@6b1e8c7\#399825:gh41042213661.1
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/41042213661 (attempt 1)
## v1.0.1793175192005

- [e2e][release][negative] a tag collision at another merge commit is rejected during planning — 1.0.1793174793864 (#1366)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, release planning rejects a tag pointing to another merge commit (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f

- [e2e][release][negative] rejects a pre-existing release authored outside the Server App — 1.0.1793175192005 (#1367)

## E2E test
- Scenario: release.foreign-release — rejects a pre-existing release authored outside the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge and expected build failure, publication refuses the owner-authored draft release (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793174793864

- [e2e][release][positive] Publish the seed release for the tag collision test — 1.0.1793174793864 (#1365)

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: seed-release
- Classification: positive
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Server App deliberately merges the seed PR (permitted).
2. release: Publication creates the existing immutable tag used by the next PR (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793174596710

- [e2e][release][negative] a stale committed dist file fails the js-action build and still cleans up the request — 1.0.1793174596710 (#1364)

## E2E test
- Scenario: release.stale-dist — a stale committed dist file fails the js-action build and still cleans up the request
- Case: fixture
- Classification: negative
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful Server App setup merge, stale dist fails the build and publication is skipped (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793174316037

- [e2e][release][positive] replaying a completed none release preserves the immutable release and asset digests — 1.0.1793174316037 (#1363)

## E2E test
- Scenario: release.none-replay — replaying a completed none release preserves the immutable release and asset digests
- Case: fixture
- Classification: positive
- Checks: Release publication; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves the immutable release and asset digests (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793173740960

- [e2e][release][positive] publishes and replays an immutable terraform-provider release with all seventeen verified assets — 1.0.1793173740960 (#1362)

## E2E test
- Scenario: release.terraform-replay — publishes and replays an immutable terraform-provider release with all seventeen verified assets
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves all seventeen verified immutable assets (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793173443788

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1793173443788 (#1361)

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793173159108

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1793173159108 (#1360)

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793172943028

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1793172943028 (#1359)

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793172719695

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1793172719695 (#1358)

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793172482553

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1793172482553 (#1357)

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#ad453f
- Runner: Local invocation release@c22b1d7\#ad453f
## v1.0.1793170589519

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1793170589519 (#1356)

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#8cb1e3
- Runner: Local invocation release@c22b1d7\#8cb1e3
## v1.0.1793170316795

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1793170316795 (#1355)

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#8cb1e3
- Runner: Local invocation release@c22b1d7\#8cb1e3
## v1.0.1793170080104

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1793170080104 (#1354)

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#8cb1e3
- Runner: Local invocation release@c22b1d7\#8cb1e3
## v1.0.1793169842406

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1793169842406 (#1353)

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#8cb1e3
- Runner: Local invocation release@c22b1d7\#8cb1e3
## v1.0.1793169618816

- Sync csm-ci e2e fixture .release-version

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1793169618816 (#1352)

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Harness SHA: c22b1d76ceb9b45ee72d46712135909f605f6d94
- Invocation: release@c22b1d7\#8cb1e3
- Runner: Local invocation release@c22b1d7\#8cb1e3
## v1.0.1793167125342

- [e2e][release][negative] rejects an invalid release version during planning — invalid-1793106241486

## E2E test
- Scenario: release.invalid-version — rejects an invalid release version during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Request validation

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, planning rejects the invalid committed version (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture herdr-plugin.toml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release-pr.yml

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvqtm21c-nw1f

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 45cb211c35490ac9d37d1056023bb4b66ce424b0
- Harness SHA: 45cb211c35490ac9d37d1056023bb4b66ce424b0
- Invocation: merge@45cb211\#b5965d
- Runner: Local invocation merge@45cb211\#b5965d

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvqumncj-ynn9

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2f07fd858e1d090269618beb68155fc78b572480
- Harness SHA: 2f07fd858e1d090269618beb68155fc78b572480
- Invocation: merge@2f07fd8\#e3ec5c
- Runner: Local invocation merge@2f07fd8\#e3ec5c

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvqv3g13-y0y3

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2f07fd858e1d090269618beb68155fc78b572480
- Harness SHA: 2f07fd858e1d090269618beb68155fc78b572480
- Invocation: merge@2f07fd8\#e3ec5c
- Runner: Local invocation merge@2f07fd8\#e3ec5c

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvqvkr3c-th50

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 2f07fd858e1d090269618beb68155fc78b572480
- Harness SHA: 2f07fd858e1d090269618beb68155fc78b572480
- Invocation: merge@2f07fd8\#e3ec5c
- Runner: Local invocation merge@2f07fd8\#e3ec5c

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvqwm6wx-gc1v

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6cd63e33b51f088bcd6f777b9cc675980c3ffdc5
- Harness SHA: 6cd63e33b51f088bcd6f777b9cc675980c3ffdc5
- Invocation: approve,merge,policy-check,smoke@6cd63e3\#3de625
- Runner: Local invocation approve,merge,policy-check,smoke@6cd63e3\#3de625

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvqwxpjx-887a

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6cd63e33b51f088bcd6f777b9cc675980c3ffdc5
- Harness SHA: 6cd63e33b51f088bcd6f777b9cc675980c3ffdc5
- Invocation: approve,merge,policy-check,smoke@6cd63e3\#3de625
- Runner: Local invocation approve,merge,policy-check,smoke@6cd63e3\#3de625

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvqx0xoj-bbou

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6cd63e33b51f088bcd6f777b9cc675980c3ffdc5
- Harness SHA: 6cd63e33b51f088bcd6f777b9cc675980c3ffdc5
- Invocation: approve,merge,policy-check,smoke@6cd63e3\#3de625
- Runner: Local invocation approve,merge,policy-check,smoke@6cd63e3\#3de625

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvqzkw1q-v3ku (#1137)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 398a5c0d7bdd02768b88fef17af7bcc0c770896d
- Harness SHA: 398a5c0d7bdd02768b88fef17af7bcc0c770896d
- Invocation: approve,merge,policy-check,smoke@398a5c0\#1e570d
- Runner: Local invocation approve,merge,policy-check,smoke@398a5c0\#1e570d

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvqzttaw-ed4j (#1148)

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 398a5c0d7bdd02768b88fef17af7bcc0c770896d
- Harness SHA: 398a5c0d7bdd02768b88fef17af7bcc0c770896d
- Invocation: approve,merge,policy-check,smoke@398a5c0\#1e570d
- Runner: Local invocation approve,merge,policy-check,smoke@398a5c0\#1e570d

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvqzy261-1j6q (#1149)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 398a5c0d7bdd02768b88fef17af7bcc0c770896d
- Harness SHA: 398a5c0d7bdd02768b88fef17af7bcc0c770896d
- Invocation: approve,merge,policy-check,smoke@398a5c0\#1e570d
- Runner: Local invocation approve,merge,policy-check,smoke@398a5c0\#1e570d

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvr220ds-cuvd (#1189)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 95a3bd6782f93aee009f2c35e4299553a5190265
- Harness SHA: 95a3bd6782f93aee009f2c35e4299553a5190265
- Invocation: approve,merge,policy-check,securefix,smo@95a3bd6\#0800eb
- Runner: Local invocation approve,merge,policy-check,securefix,smo@95a3bd6\#0800eb

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvr2ij68-jp6u (#1208)

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 95a3bd6782f93aee009f2c35e4299553a5190265
- Harness SHA: 95a3bd6782f93aee009f2c35e4299553a5190265
- Invocation: approve,merge,policy-check,securefix,smo@95a3bd6\#0800eb
- Runner: Local invocation approve,merge,policy-check,securefix,smo@95a3bd6\#0800eb

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvr2m0ns-1nzk (#1209)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 95a3bd6782f93aee009f2c35e4299553a5190265
- Harness SHA: 95a3bd6782f93aee009f2c35e4299553a5190265
- Invocation: approve,merge,policy-check,securefix,smo@95a3bd6\#0800eb
- Runner: Local invocation approve,merge,policy-check,securefix,smo@95a3bd6\#0800eb

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvr3uxu5-qqua (#1242)

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 0479c9974f6ad44e60ca715fdfba76941730ba00
- Harness SHA: 0479c9974f6ad44e60ca715fdfba76941730ba00
- Invocation: approve,merge,policy-check,securefix,smo@0479c99\#f5038b
- Runner: Local invocation approve,merge,policy-check,securefix,smo@0479c99\#f5038b

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvr5bl45-nlts (#1255)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 45af3fbe616e74a63a2c6aeec1c03fc02de8e3e9
- Harness SHA: 45af3fbe616e74a63a2c6aeec1c03fc02de8e3e9
- Invocation: approve,merge,policy-check,securefix,smo@45af3fb\#62b86b
- Runner: Local invocation approve,merge,policy-check,securefix,smo@45af3fb\#62b86b

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvr5kasy-kfti (#1276)

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 45af3fbe616e74a63a2c6aeec1c03fc02de8e3e9
- Harness SHA: 45af3fbe616e74a63a2c6aeec1c03fc02de8e3e9
- Invocation: approve,merge,policy-check,securefix,smo@45af3fb\#62b86b
- Runner: Local invocation approve,merge,policy-check,securefix,smo@45af3fb\#62b86b

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvr5n9lw-8grn (#1277)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 45af3fbe616e74a63a2c6aeec1c03fc02de8e3e9
- Harness SHA: 45af3fbe616e74a63a2c6aeec1c03fc02de8e3e9
- Invocation: approve,merge,policy-check,securefix,smo@45af3fb\#62b86b
- Runner: Local invocation approve,merge,policy-check,securefix,smo@45af3fb\#62b86b

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvr6rukt-4l5w (#1293)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Harness SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Invocation: approve,merge,policy-check,securefix,smo@8e0c2b9\#c85fd5
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40585359273 (attempt 1)

- [e2e][merge][positive] /merge posted before /approve on the same pull request still merges — e2e-mvr72slb-fkiy (#1319)

## E2E test
- Scenario: merge.concurrent-approve — /merge posted before /approve on the same pull request still merges
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: A waiting /merge does not block /approve on the same pull request, and the merge completes (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Harness SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Invocation: approve,merge,policy-check,securefix,smo@8e0c2b9\#c85fd5
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40585359273 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvr76dpc-xtx6 (#1320)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Harness SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Invocation: approve,merge,policy-check,securefix,smo@8e0c2b9\#c85fd5
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40585359273 (attempt 1)

- [e2e][merge][positive] Configured Bot merges its own PR — e2e-mvr7nz5f-lojy (#1326)

## E2E test
- Scenario: merge.configured-bot-ownership — a configured bot can merge its own pull request but not another author's pull request or commit
- Case: own-pr
- Classification: positive
- Checks: Bot PR and commit ownership; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The configured Bot request succeeds and the Server App merges this PR. (permitted).

## Identity and trust during this case
- PR author, effective commit identity and command requester: testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\).
- The test temporarily adds testing-csm-ci-untrusted-committer\[bot\] \(ID 347402450\) to trust.committers and trust.bot\_mergers. The repository permits Bot requests and exempts the owner and this Bot from its all-path sensitivity. The harness attempts to restore the original configuration.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Harness SHA: 8e0c2b90bca145b1f22d55f5a4ffe4a3e71edc53
- Invocation: approve,merge,policy-check,securefix,smo@8e0c2b9\#c85fd5
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40585359273 (attempt 1)

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvrmgrip-98me (#1340)

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 65cf39d8589599164adf76b0a2e8bf614318b306
- Harness SHA: 65cf39d8589599164adf76b0a2e8bf614318b306
- Invocation: merge@65cf39d\#f04a40
- Runner: Local invocation merge@65cf39d\#f04a40

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvrmzv5v-lrt8 (#1346)

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 65cf39d8589599164adf76b0a2e8bf614318b306
- Harness SHA: 65cf39d8589599164adf76b0a2e8bf614318b306
- Invocation: merge@65cf39d\#f04a40
- Runner: Local invocation merge@65cf39d\#f04a40

- [e2e][merge][positive] /merge before /approve enables platform auto-merge and GitHub merges after approval — e2e-mvrnepuh-bw3a (#1349)

## E2E test
- Scenario: merge.concurrent-approve — /merge before /approve enables platform auto-merge and GitHub merges after approval
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: /merge enables platform auto-merge before approval, and GitHub merges once /approve lands (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 07976fadd9367f3b8300abe157e181e9c819487d
- Harness SHA: 07976fadd9367f3b8300abe157e181e9c819487d
- Invocation: merge@07976fa\#7d168c
- Runner: Local invocation merge@07976fa\#7d168c

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1793167125342 (#1351)

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: a45bb77a2ecf30fce3ce2aecdce9ec8139138508
- Harness SHA: a45bb77a2ecf30fce3ce2aecdce9ec8139138508
- Invocation: release@a45bb77\#260225
- Runner: Local invocation release@a45bb77\#260225
## v1.0.1793105950559

- [e2e][release][negative] a tag collision at another merge commit is rejected during planning — 1.0.1793105560211

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, release planning rejects a tag pointing to another merge commit (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)

- [e2e][release][negative] rejects a pre-existing release authored outside the Server App — 1.0.1793105950559

## E2E test
- Scenario: release.foreign-release — rejects a pre-existing release authored outside the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge and expected build failure, publication refuses the owner-authored draft release (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793105560211

- [e2e][release][positive] Publish the seed release for the tag collision test — 1.0.1793105560211

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: seed-release
- Classification: positive
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Server App deliberately merges the seed PR (permitted).
2. release: Publication creates the existing immutable tag used by the next PR (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793105333066

- [e2e][release][negative] a stale committed dist file fails the js-action build and still cleans up the request — 1.0.1793105333066

## E2E test
- Scenario: release.stale-dist — a stale committed dist file fails the js-action build and still cleans up the request
- Case: fixture
- Classification: negative
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful Server App setup merge, stale dist fails the build and publication is skipped (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793104906753

- [e2e][release][positive] replaying a completed none release preserves the immutable release and asset digests — 1.0.1793104906753

## E2E test
- Scenario: release.none-replay — replaying a completed none release preserves the immutable release and asset digests
- Case: fixture
- Classification: positive
- Checks: Release publication; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves the immutable release and asset digests (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793104308366

- [e2e][release][positive] publishes and replays an immutable terraform-provider release with all seventeen verified assets — 1.0.1793104308366

## E2E test
- Scenario: release.terraform-replay — publishes and replays an immutable terraform-provider release with all seventeen verified assets
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves all seventeen verified immutable assets (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793103986282

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1793103986282

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793103700821

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1793103700821

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793103460175

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1793103460175

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793103235111

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1793103235111

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793103010458

- [e2e][release][negative] rejects an invalid release version during planning — invalid-1793088382304

## E2E test
- Scenario: release.invalid-version — rejects an invalid release version during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Request validation

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, planning rejects the invalid committed version (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161

- [e2e][release][negative] rejects a pull request merged by the owner instead of the Server App — 1.0.1793088566179 (#957)

## E2E test
- Scenario: release.wrong-merger — rejects a pull request merged by the
owner instead of the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization
evidence. Consult the execution and operation receipts for actual
results.

1. release: The owner intentionally merges the setup PR; release
planning rejects it because the Server App did not merge it (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed
commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations
originate from the fixture workflow. Trust is configured by immutable
IDs.

The harness attempts to restore temporary test configuration during
cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture .github/workflows/csm-ci-policy-check.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-merge.yml

- e2e e2e-mvqg8auy-a1v5

- e2e e2e-mvqgdtva-sbja

- Sync csm-ci e2e fixture .github/workflows/csm-ci-policy-check.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-merge.yml

- exercise inferred release PR version

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvqlieub-v8mp

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: merge@6c95a5e\#dc1770
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40495543341 (attempt 1)

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvqlneug-xtnl

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: merge@6c95a5e\#dc1770
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40495543341 (attempt 1)

- exercise inferred release PR version

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1793103010458

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Harness SHA: 6c95a5e9b35e8eeba8e0c6f78055acc38b698bb4
- Invocation: release@6c95a5e\#55c255
- Runner: https://github.com/civitaspo/csm-ci-actions/actions/runs/40501171602 (attempt 1)
## v1.0.1793088124793

- [e2e][release][negative] a tag collision at another merge commit is rejected during planning — 1.0.1793087760629

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: fixture
- Classification: negative
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge, release planning rejects a tag pointing to another merge commit (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161

- [e2e][release][negative] rejects a pre-existing release authored outside the Server App — 1.0.1793088124793

## E2E test
- Scenario: release.foreign-release — rejects a pre-existing release authored outside the Server App
- Case: fixture
- Classification: negative
- Checks: Release publication; Immutable identities and configured trust

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful setup merge and expected build failure, publication refuses the owner-authored draft release (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793087760629

- [e2e][release][positive] Publish the seed release for the tag collision test — 1.0.1793087760629

## E2E test
- Scenario: release.tag-collision — a tag collision at another merge commit is rejected during planning
- Case: seed-release
- Classification: positive
- Checks: Release publication; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Server App deliberately merges the seed PR (permitted).
2. release: Publication creates the existing immutable tag used by the next PR (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793087578057

- [e2e][release][negative] a stale committed dist file fails the js-action build and still cleans up the request — 1.0.1793087578057

## E2E test
- Scenario: release.stale-dist — a stale committed dist file fails the js-action build and still cleans up the request
- Case: fixture
- Classification: negative
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: After an intentionally successful Server App setup merge, stale dist fails the build and publication is skipped (rejected).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793087308108

- [e2e][release][positive] replaying a completed none release preserves the immutable release and asset digests — 1.0.1793087308108

## E2E test
- Scenario: release.none-replay — replaying a completed none release preserves the immutable release and asset digests
- Case: fixture
- Classification: positive
- Checks: Release publication; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves the immutable release and asset digests (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793086720800

- [e2e][release][positive] publishes and replays an immutable terraform-provider release with all seventeen verified assets — 1.0.1793086720800

## E2E test
- Scenario: release.terraform-replay — publishes and replays an immutable terraform-provider release with all seventeen verified assets
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations; Repeated requests and receipts

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR merges, publication succeeds, and replay preserves all seventeen verified immutable assets (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793086418673

- [e2e][release][positive] publishes an immutable cargo-raw release with verified asset attestations — 1.0.1793086418673

## E2E test
- Scenario: release.publish-cargo-raw — publishes an immutable cargo-raw release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793086135949

- [e2e][release][positive] publishes an immutable cargo release with verified asset attestations — 1.0.1793086135949

## E2E test
- Scenario: release.publish-cargo — publishes an immutable cargo release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793085894946

- [e2e][release][positive] publishes an immutable goreleaser release with verified asset attestations — 1.0.1793085894946

## E2E test
- Scenario: release.publish-goreleaser — publishes an immutable goreleaser release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793085671825

- [e2e][release][positive] publishes an immutable js-action release with verified asset attestations — 1.0.1793085671825

## E2E test
- Scenario: release.publish-js-action — publishes an immutable js-action release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793085450023

- e2e release invalid-1793042970201

- e2e release 1.0.1793043123974 (#761)

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- exercise inferred release PR version

- validate signed funding fixture heads

- Sync managed files

Update files managed by csm-ci.

<!-- csm-ci:repo-settings:managed-files -->

- restore funding probe baseline

- exercise inferred release PR version

- validate protected release fixture heads

- v1.1.1793065268072

- restore release title acceptance fixture

- exercise inferred release PR version

- remove the issue 69 release fixture CI

- e2e e2e-mvq6ckb7-v5o8

- e2e e2e-mvq6hym4-xoz5

- e2e e2e-mvq6uubu-wsga

- e2e e2e-mvq6zprh-vqj8

- exercise inferred release PR version

- e2e e2e-mvq83we0-okfe

- e2e e2e-mvq88fjq-svqj

- e2e e2e-mvq92xoa-e6jv

- e2e e2e-mvq98et4-0eid

- exercise inferred release PR version

- Sync csm-ci e2e fixture .github/workflows/csm-ci-policy-check.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-merge.yml

- [e2e][merge][positive] /merge by an authorized merger squashes the approved head and posts one receipt — e2e-mvqakszd-d62m

## E2E test
- Scenario: merge.authorized — /merge by an authorized merger squashes the approved head and posts one receipt
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: The authorized command squashes the approved head and posts one receipt (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: merge@3fae8cb\#03234f
- Runner: Local invocation merge@3fae8cb\#03234f

- [e2e][merge][positive] /merge squashes an explicitly approved workflow change at the pinned head — e2e-mvqaqbdt-tqch

## E2E test
- Scenario: merge.workflow-change — /merge squashes an explicitly approved workflow change at the pinned head
- Case: fixture
- Classification: positive
- Checks: Approval and policy results; Allowed paths and changes; Pinned head consistency

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. merge: Explicit approval permits the workflow change and merge at its pinned head (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: merge
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: merge@3fae8cb\#03234f
- Runner: Local invocation merge@3fae8cb\#03234f

- Sync csm-ci e2e fixture .github/workflows/csm-ci-policy-check.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-merge.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-policy-check.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-merge.yml

- exercise inferred release PR version

- [e2e][release][positive] publishes an immutable none release with verified asset attestations — 1.0.1793085450023

## E2E test
- Scenario: release.publish-none — publishes an immutable none release with verified asset attestations
- Case: fixture
- Classification: positive
- Checks: Release publication; Release assets and attestations

## Expected behavior
These are test declarations, not observed results or authorization evidence. Consult the execution and operation receipts for actual results.

1. release: The setup PR is deliberately merged by the Server App, then release publication and asset verification succeed (permitted).

## Identity and trust during this case
- PR author: civitaspo \(ID 4525500\).
- Effective commit identity: civitaspo \(ID 4525500\); GitHub-signed commits use the owner as author and web-flow as committer.
- Command requester: civitaspo \(ID 4525500\); automatic operations originate from the fixture workflow. Trust is configured by immutable IDs.

The harness attempts to restore temporary test configuration during cleanup; this description does not claim that cleanup succeeded.

## Execution
- Feature: release
- Candidate SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Harness SHA: 3fae8cb6f597c2c4f54d688ccc3768f084db60ae
- Invocation: release@3fae8cb\#088161
- Runner: Local invocation release@3fae8cb\#088161
## v1.0.1793042727503

- e2e release 1.0.1793042376660

- e2e release 1.0.1793042727503
## v1.0.1793042376660

- e2e release 1.0.1793042376660
## v1.0.1793042199736

- e2e release 1.0.1793042199736
## v1.0.1793041927666

- e2e release 1.0.1793041927666
## v1.0.1793041311706

- e2e release 1.0.1793041311706
## v1.0.1793040988496

- e2e release 1.0.1793040988496
## v1.0.1793040708862

- e2e release 1.0.1793040708862
## v1.0.1793040452300

- e2e release 1.0.1793040452300
## v1.0.1793040230609

- e2e release 1.0.1793040230609
## v1.0.1793040010484

- e2e release invalid-1793038226865

- e2e release 1.0.1793038428520 (#747)

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1793040010484
## v1.0.1793037910889

- e2e release 1.0.1793037362492

- e2e release 1.0.1793037910889
## v1.0.1793037362492

- e2e release 1.0.1793037362492
## v1.0.1793037154685

- e2e release 1.0.1793037154685
## v1.0.1793036850106

- e2e release 1.0.1793036850106
## v1.0.1793036249024

- e2e release 1.0.1793036249024
## v1.0.1793035893836

- e2e release 1.0.1793035893836
## v1.0.1793035588841

- e2e release 1.0.1793035588841
## v1.0.1793035318348

- e2e release 1.0.1793035318348
## v1.0.1793034581238

- e2e release 1.0.1793034581238
## v1.0.1793034350714

- e2e release invalid-1793028272742

- e2e release 1.0.1793028437068 (#695)

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1793034350714
## v1.0.1793027983941

- e2e release 1.0.1793027578561

- e2e release 1.0.1793027983941
## v1.0.1793027578561

- e2e release 1.0.1793027578561
## v1.0.1793027396694

- e2e release 1.0.1793027396694
## v1.0.1793027095795

- e2e release 1.0.1793027095795
## v1.0.1793026499557

- e2e release 1.0.1793026499557
## v1.0.1793025921028

- e2e release 1.0.1793025921028
## v1.0.1793025634414

- e2e release 1.0.1793025634414
## v1.0.1793025384759

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1793025384759
## v1.0.1793024080262

- e2e release invalid-1793018465517

- e2e release 1.0.1793018631282 (#661)

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e e2e-mvp9ax53-dodq

- e2e e2e-mvp9g2gs-u9iy

- e2e e2e-mvpaupdi-y65r

- e2e e2e-mvpb02sk-s5xg

- e2e e2e-mvpbfxme-8btp

- e2e release 1.0.1793024080262
## v1.0.1793018206164

- e2e release 1.0.1793017857084

- e2e release 1.0.1793018206164
## v1.0.1793017857084

- e2e release 1.0.1793017857084
## v1.0.1793017694894

- e2e release 1.0.1793017694894
## v1.0.1793017438752

- e2e release 1.0.1793017438752
## v1.0.1793017163821

- e2e release 1.0.1793017163821
## v1.0.1793016849082

- e2e release 1.0.1793016849082
## v1.0.1793016595447

- e2e release 1.0.1793016595447
## v1.0.1793016362445

- e2e release 1.0.1793016362445
## v1.0.1793016146192

- e2e release 1.0.1793016146192
## v1.0.1793015921838

- e2e release invalid-1793011824608

- e2e release 1.0.1793011995693 (#648)

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1793015921838
## v1.0.1793011560014

- e2e release 1.0.1793011201405

- e2e release 1.0.1793011560014
## v1.0.1793011201405

- e2e release 1.0.1793011201405
## v1.0.1793011023110

- e2e release 1.0.1793011023110
## v1.0.1793010744744

- e2e release 1.0.1793010744744
## v1.0.1793010190645

- e2e release 1.0.1793010190645
## v1.0.1793009894722

- e2e release 1.0.1793009894722
## v1.0.1793009622017

- e2e release 1.0.1793009622017
## v1.0.1793009390597

- e2e release 1.0.1793009390597
## v1.0.1793009178835

- e2e release 1.0.1793009178835
## v1.0.1793008971038

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1793008971038
## v1.0.1793007943009

- e2e release invalid-1793007484742

- e2e release 1.0.1793007648481 (#634)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1793007943009
## v1.0.1793007217025

- e2e release 1.0.1793006854359

- e2e release 1.0.1793007217025
## v1.0.1793006854359

- e2e release 1.0.1793006854359
## v1.0.1793006670359

- e2e release 1.0.1793006670359
## v1.0.1793006393778

- e2e release 1.0.1793006393778
## v1.0.1793005892963

- e2e release 1.0.1793005892963
## v1.0.1793005619183

- e2e release 1.0.1793005619183
## v1.0.1793005389248

- e2e release 1.0.1793005389248
## v1.0.1793005176780

- e2e release 1.0.1793005176780
## v1.0.1793004973906

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1793004973906
## v1.0.1793004076525

- e2e release 1.0.1793004076525
## v1.0.1793003865199

- e2e release 1.0.1793003865199
## v1.0.1793003628534

- e2e release 1.0.1793003628534
## v1.0.1793003414029

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e e2e-mvol3cd1-pcxs

- e2e e2e-mvoljryp-czsf

- e2e e2e-mvolytza-34cv

- exercise inferred release PR version

- e2e release 1.0.1793003414029
## v1.0.1792972947495

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792972947495
## v1.0.1792972407414

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792972407414
## v1.0.1792972152202

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792972152202
## v1.0.1792968031827

- Sync csm-ci e2e fixture .release-version

- e2e e2e-mvob0r1k-lrq2

- e2e release 1.0.1792968031827
## v1.0.1792960288109

- Sync csm-ci e2e fixture .release-version

- exercise inferred release PR version

- e2e release 1.0.1792960288109
## v1.0.1792958270379

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792958270379
## v1.0.1792957989380

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture pyproject.toml

- Sync csm-ci e2e fixture dbt_project.yml

- exercise inferred release PR version

- Sync csm-ci e2e fixture build.rs

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release-pr.yml

- Sync csm-ci e2e fixture src/main.rs

- e2e release 1.0.1792957989380
## v1.0.1792940633369

- e2e release 1.0.1792940633369
## v1.0.1792940378578

- e2e release 1.0.1792940378578
## v1.0.1792940158714

- e2e release 1.0.1792940158714
## v1.0.1792939953505

- e2e release 1.0.1792939953505
## v1.0.1792939740837

- e2e release invalid-1792926799122

- e2e release 1.0.1792926943650 (#521)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- exercise inferred release PR version

- exercise inferred release PR version

- exercise inferred release PR version

- exercise inferred release PR version

- e2e release 1.0.1792939740837
## v1.0.1792926560674

- e2e release 1.0.1792926233531

- e2e release 1.0.1792926560674
## v1.0.1792926233531

- e2e release 1.0.1792926233531
## v1.0.1792926048903

- e2e release 1.0.1792926048903
## v1.0.1792925802892

- e2e release 1.0.1792925802892
## v1.0.1792925545022

- e2e release 1.0.1792925545022
## v1.0.1792925345104

- e2e release 1.0.1792925345104
## v1.0.1792925161086

- e2e release 1.0.1792925161086
## v1.0.1792924961405

- e2e release invalid-1792921913393

- e2e release 1.0.1792922062286 (#464)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- exercise inferred release PR version

- exercise inferred release PR version

- e2e e2e-mvnoaxqt-xlgq

- e2e e2e-mvnopdnm-p1rz

- e2e release 1.0.1792924961405
## v1.0.1792921659974

- e2e release 1.0.1792921303473

- e2e release 1.0.1792921659974
## v1.0.1792921303473

- e2e release 1.0.1792921303473
## v1.0.1792921126832

- e2e release 1.0.1792921126832
## v1.0.1792920849587

- e2e release 1.0.1792920849587
## v1.0.1792920591018

- e2e release 1.0.1792920591018
## v1.0.1792920379466

- e2e release 1.0.1792920379466
## v1.0.1792920173534

- e2e release 1.0.1792920173534
## v1.0.1792919982732

- e2e release invalid-1792891636042

- e2e release 1.0.1792891771685 (#324)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture cliff.toml

- Sync csm-ci e2e fixture package.json

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release-pr.yml

- Sync csm-ci e2e fixture cliff.toml

- exercise inferred release PR version

- exercise inferred release PR version

- exercise inferred release PR version

- exercise inferred release PR version

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-route.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-forged.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix.yml

- exercise inferred release PR version

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-route.yml

- exercise inferred release PR version

- exercise inferred release PR version

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-route.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix.yml

- exercise inferred release PR version

- exercise inferred release PR version

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release-pr.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-forged.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- e2e e2e-mvnll3v9-9ole

- e2e release 1.0.1792919982732
## v1.0.1792891403007

- e2e release 1.0.1792891063811

- e2e release 1.0.1792891403007
## v1.0.1792891063811

- e2e release 1.0.1792891063811
## v1.0.1792890905958

- e2e release 1.0.1792890905958
## v1.0.1792890668145

- e2e release 1.0.1792890668145
## v1.0.1792890389371

- e2e release 1.0.1792890389371
## v1.0.1792890179810

- e2e release 1.0.1792890179810
## v1.0.1792889978024

- e2e release 1.0.1792889978024
## v1.0.1792889778301

- e2e release invalid-1792889148365

- e2e release 1.0.1792889290952 (#313)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792889778301
## v1.0.1792888915145

- e2e release 1.0.1792888596595

- e2e release 1.0.1792888915145
## v1.0.1792888596595

- e2e release 1.0.1792888596595
## v1.0.1792888424683

- e2e release 1.0.1792888424683
## v1.0.1792888184935

- e2e release 1.0.1792888184935
## v1.0.1792887943976

- e2e release 1.0.1792887943976
## v1.0.1792887733818

- e2e release 1.0.1792887733818
## v1.0.1792887532104

- e2e release 1.0.1792887532104
## v1.0.1792887325491

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792887325491
## v1.0.1792887054127

- e2e release invalid-1792886567440

- e2e release 1.0.1792886721683 (#301)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792887054127
## v1.0.1792886337166

- e2e release 1.0.1792886008292

- e2e release 1.0.1792886337166
## v1.0.1792886008292

- e2e release 1.0.1792886008292
## v1.0.1792885853821

- e2e release 1.0.1792885853821
## v1.0.1792885604731

- e2e release 1.0.1792885604731
## v1.0.1792885346917

- e2e release 1.0.1792885346917
## v1.0.1792885142711

- e2e release 1.0.1792885142711
## v1.0.1792884933101

- e2e release 1.0.1792884933101
## v1.0.1792884756055

- e2e release invalid-1792884207107

- e2e release 1.0.1792884368464 (#290)

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792884756055
## v1.0.1792883960030

- e2e release 1.0.1792883625181

- e2e release 1.0.1792883960030
## v1.0.1792883625181

- e2e release 1.0.1792883625181
## v1.0.1792883464876

- e2e release 1.0.1792883464876
## v1.0.1792883213334

- e2e release 1.0.1792883213334
## v1.0.1792882954055

- e2e release 1.0.1792882954055
## v1.0.1792882751439

- e2e release 1.0.1792882751439
## v1.0.1792882545799

- e2e release 1.0.1792882545799
## v1.0.1792882339241

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792882339241
## v1.0.1792881990950

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792881990950
## v1.0.1792881745845

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792881745845
## v1.0.1792881541521

- e2e release 1.0.1792881541521
## v1.0.1792881367376

- e2e release 1.0.1792881367376
## v1.0.1792881116218

- e2e release 1.0.1792881116218
## v1.0.1792880857668

- e2e release 1.0.1792880857668
## v1.0.1792880697754

- e2e release 1.0.1792880697754
## v1.0.1792880456723

- e2e release 1.0.1792880456723
## v1.0.1792880263093

- e2e release invalid-1792879806832

- e2e release 1.0.1792879968533 (#270)

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792880263093
## v1.0.1792879569142

- e2e release 1.0.1792879207127

- e2e release 1.0.1792879569142
## v1.0.1792879207127

- e2e release 1.0.1792879207127
## v1.0.1792879040931

- e2e release 1.0.1792879040931
## v1.0.1792878778675

- e2e release 1.0.1792878778675
## v1.0.1792878598839

- e2e release 1.0.1792878598839
## v1.0.1792878418366

- e2e release 1.0.1792878418366
## v1.0.1792878176012

- e2e release 1.0.1792878176012
## v1.0.1792877966069

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .release-version

- e2e release 1.0.1792877966069
## v1.0.1792877600028

- Add temporary csm-ci App installation probe

- Fix probe workflow YAML

- Remove temporary csm-ci App installation probe

- Add temporary context probe

- Remove temporary context probe

- Sync csm-ci e2e fixture .github/workflows/csm-ci-probe.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-probe.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- approval by an App (#1)

- spike-approve-2 (#2)

- spike-approve-3 (#3)

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-approve.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-smoke.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-policy-check.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-merge.yml

- e2e e2e-mvmtd8ct-tloq

- e2e e2e-mvmtrz34-mwks

- e2e e2e-mvmu6vp0-xl8q

- e2e e2e-mvmuccq9-kqqq

- e2e e2e-mvmurne1-3drd

- e2e e2e-mvmv60jt-tbhv

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-route.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-forged.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix-target.yml

- Sync csm-ci e2e fixture .github/workflows/csm-ci-securefix.yml

- Sync csm-ci e2e fixture e2e/securefix-version.json

- Sync csm-ci e2e fixture cmd/release-fixture/main.go

- Sync csm-ci e2e fixture Cargo.toml

- Sync csm-ci e2e fixture go.mod

- Sync csm-ci e2e fixture .goreleaser.yaml

- Sync csm-ci e2e fixture dist/action.js

- Sync csm-ci e2e fixture Cargo.lock

- Sync csm-ci e2e fixture package.json

- Sync csm-ci e2e fixture scripts/build.mjs

- Sync csm-ci e2e fixture .github/workflows/csm-ci-release.yml

- Sync csm-ci e2e fixture .release-version

- Sync csm-ci e2e fixture src/action.js

- Sync csm-ci e2e fixture src/main.rs

- e2e release 1.0.1792877600028
