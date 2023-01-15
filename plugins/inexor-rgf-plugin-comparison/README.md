# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module  | Functionality                                                        | Tests                                                                                                                                                            |
|---------------------|--------|-------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Comparison  | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-comparison">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-comparison) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-comparison/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin provides the type system and behaviour for comparisons of two inputs.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-plugin-comparison/Rust">](https://github.com/inexorgame/inexor-rgf-plugin-comparison/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-comparison">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-comparison">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-comparison">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-comparison)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-comparison">](https://github.com/inexorgame/inexor-rgf-plugin-comparison/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Components

| Name            | Description                         | Property | Data Type   | Socket Type |
|-----------------|-------------------------------------|----------|-------------|-------------|
| comparison_gate | Compares two input values (lhs,rhs) | lhs      | any/depends | input       |
|                 |                                     | rhs      | any/depends | input       |
|                 |                                     | result   | any/depends | output      |

#### Entity Types

| Name                   | Components      | Description                                                            |
|------------------------|-----------------|------------------------------------------------------------------------|
| equals                 | comparison_gate | Returns true, if lhs and rhs are equal                                 |
| greater_than           | comparison_gate | Returns true, if the value of lhs is greater than the value of rhs     |
| greater_than_or_equals | comparison_gate | Returns true, if value of lhs is greater than or equal to value of rhs |
| lower_than             | comparison_gate | Returns true, if value of lhs is lower than value of rhs               |
| lower_than_or_equals   | comparison_gate | Returns true, if value of lhs is lower than or equal to value of rhs   |
| not_equals             | comparison_gate | Returns true, if lhs and rhs are not equal                             |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                                  |           |                                                                   |
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-comparison/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
