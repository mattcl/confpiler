# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.1 (2022-04-11)

### Bug Fixes

 - <csr-id-899bd6a4caa5f74acaa8dc21686fbcf8ba0bd961/> use rustls feature for self_update to avoid openssl musl pain

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - use rustls feature for self_update to avoid openssl musl pain ([`899bd6a`](https://github.comgit//mattcl/confpiler/commit/899bd6a4caa5f74acaa8dc21686fbcf8ba0bd961))
</details>

## 0.4.0 (2022-04-11)

### New Features

 - <csr-id-88a6565ca1306eec34de219a54d21445590c7d5c/> `update` command for self updates

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release confpiler_cli v0.4.0 ([`3510338`](https://github.comgit//mattcl/confpiler/commit/35103380caf2d5612a460360b8e898d9c92d5f70))
    - `update` command for self updates ([`88a6565`](https://github.comgit//mattcl/confpiler/commit/88a6565ca1306eec34de219a54d21445590c7d5c))
</details>

## 0.3.4 (2022-04-11)

<csr-id-f2728d887b88feabaf0d53a1e7bd787f288ea652/>

### Refactor

 - <csr-id-f2728d887b88feabaf0d53a1e7bd787f288ea652/> move cli config getting into args

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release confpiler_cli v0.3.4 ([`f308c6b`](https://github.comgit//mattcl/confpiler/commit/f308c6b771a26a7a7b98cf2d231d642f9b7dad49))
    - move cli config getting into args ([`f2728d8`](https://github.comgit//mattcl/confpiler/commit/f2728d887b88feabaf0d53a1e7bd787f288ea652))
</details>

## 0.3.3 (2022-04-11)

### Documentation

 - <csr-id-68ffe60270a0f32a0925e7f4a3cc4f913f7da287/> call out usage for --raw when dealing with old versions of compose

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release confpiler_cli v0.3.3 ([`15735e3`](https://github.comgit//mattcl/confpiler/commit/15735e3ff10135e60ff4ca4397bd50c583495117))
    - call out usage for --raw when dealing with old versions of compose ([`68ffe60`](https://github.comgit//mattcl/confpiler/commit/68ffe60270a0f32a0925e7f4a3cc4f913f7da287))
</details>

## 0.3.2 (2022-04-11)

### Documentation

 - <csr-id-6f9fa99f1ed03365a4a8e2eac8aecd7e0bd53edf/> make behavior of multiple directories more clear

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release confpiler_cli v0.3.2 ([`bfa0867`](https://github.comgit//mattcl/confpiler/commit/bfa08675f6d80496ed6f1ebdb56fd6d86c5cfe04))
    - make behavior of multiple directories more clear ([`6f9fa99`](https://github.comgit//mattcl/confpiler/commit/6f9fa99f1ed03365a4a8e2eac8aecd7e0bd53edf))
</details>

## 0.3.1 (2022-04-11)

<csr-id-598449234c43b8ef010c7d738086a7a7908fd3ea/>
<csr-id-0c8f530b9d670f729c0766209f80f1fb4ea05b68/>
<csr-id-8b4d0bf65e14f1f213674d717755fe94fe51a2f8/>

### Chore

 - <csr-id-8b4d0bf65e14f1f213674d717755fe94fe51a2f8/> add changelogs

### Documentation

 - <csr-id-236de63e40891b03c7b358827f1a8de670659a89/> update cargo tomls with relevant info
 - <csr-id-97b942cc898b34b8888ce1c099ad9429b297f6cc/> fixing typos, making clarifications
 - <csr-id-a683f0db860635a5cfb5917b6efd45e5e54653ca/> more tests, better readme

### New Features

 - <csr-id-a824cb042468c2ad18edc5cf045dbb99a1142bba/> allow for getting unescaped, unquoted output with --raw
 - <csr-id-9781899d3b3101eef91af431befa964c65bf87be/> allow specifying a prefix for generated keys
   This adds .with_prefix to the builder and --prefix to the CLI
 - <csr-id-10602d0361fc6e76a084021136fb0664d9897158/> first pass as cli tool
   a bunch of files were added, but they're mainly to support the
   integration tests
 - <csr-id-72375f349bb71c2bba47e23189d54f64e0a84d73/> initial commit

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 45 calendar days.
 - 10 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release confpiler v0.2.1, confpiler_cli v0.3.1 ([`e781460`](https://github.comgit//mattcl/confpiler/commit/e78146059e9b97f324dba3806edd2d2ab5a61e10))
    - add changelogs ([`8b4d0bf`](https://github.comgit//mattcl/confpiler/commit/8b4d0bf65e14f1f213674d717755fe94fe51a2f8))
    - allow for getting unescaped, unquoted output with --raw ([`a824cb0`](https://github.comgit//mattcl/confpiler/commit/a824cb042468c2ad18edc5cf045dbb99a1142bba))
    - clippy suggestions ([`5984492`](https://github.comgit//mattcl/confpiler/commit/598449234c43b8ef010c7d738086a7a7908fd3ea))
    - allow specifying a prefix for generated keys ([`9781899`](https://github.comgit//mattcl/confpiler/commit/9781899d3b3101eef91af431befa964c65bf87be))
    - exclude integration tests from cli package ([`0c8f530`](https://github.comgit//mattcl/confpiler/commit/0c8f530b9d670f729c0766209f80f1fb4ea05b68))
    - update cargo tomls with relevant info ([`236de63`](https://github.comgit//mattcl/confpiler/commit/236de63e40891b03c7b358827f1a8de670659a89))
    - fixing typos, making clarifications ([`97b942c`](https://github.comgit//mattcl/confpiler/commit/97b942cc898b34b8888ce1c099ad9429b297f6cc))
    - more tests, better readme ([`a683f0d`](https://github.comgit//mattcl/confpiler/commit/a683f0db860635a5cfb5917b6efd45e5e54653ca))
    - first pass as cli tool ([`10602d0`](https://github.comgit//mattcl/confpiler/commit/10602d0361fc6e76a084021136fb0664d9897158))
    - initial commit ([`72375f3`](https://github.comgit//mattcl/confpiler/commit/72375f349bb71c2bba47e23189d54f64e0a84d73))
</details>

