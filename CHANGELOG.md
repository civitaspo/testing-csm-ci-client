# Changelog
## v1.0.1793088124794

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
