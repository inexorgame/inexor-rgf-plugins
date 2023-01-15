# Inexor Reactive Graph Flow

| Project             | Module   | Sub-Module   | Functionality                                                        | Test Coverage                                                                                                                                                |
|---------------------|----------|--------------|----------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin   | Taxonomy     | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-taxonomy">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-taxonomy) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-taxonomy/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas...
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

### About this module

This plugin provides a graph data model for representing a taxonomy within the Reactive Graph Flow. The taxonomy enables
semantic operations on the graph like graph queries and graph navigation.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-plugin-taxonomy/Rust">](https://github.com/inexorgame/inexor-rgf-plugin-taxonomy/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-taxonomy">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-taxonomy">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-taxonomy">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-taxonomy)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-taxonomy">](https://github.com/inexorgame/inexor-rgf-plugin-taxonomy/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

## Components

| Name      | Properties | DataType | SocketType | Description                                                |
|-----------|------------|----------|------------|------------------------------------------------------------|
|           |
| weighted  | weight     | number   | none       | The weight of a relation between two entity instances      |

## Entity Types

| Name     | Components  | Properties  | DataType | SocketType | Description                     |
|----------|-------------|-------------|----------|------------|---------------------------------|
||
| category | named       | name        | string   | none       | The name of the category        |
|          | describable | description | string   | none       | The description of the category |
||
| tag      | named       | name        | string   | none       | The tag name                    |
||

## Relation Types

| Name            | Description | Components | Source Entity Type | Target Entity Type |
|-----------------|-------------|------------|--------------------|--------------------|
| categorized_as  |             |            | *                  | category           |
| has_subcategory |             |            | category           | category           |
| tagged_with     |             | weighted   | *                  | tag                |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                                |           |                                                                   |
|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-taxonomy/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
