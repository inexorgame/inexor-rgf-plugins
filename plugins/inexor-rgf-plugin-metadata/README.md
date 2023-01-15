# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Test Coverage                                                                                                                                                |
|---------------------|--------|------------|----------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | MetaData   | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-metadata">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-metadata) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-metadata/main/docs/images/inexor_2.png">
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

This plugin provides components for support of metadata. Textures, sounds, maps and other content usually has some
information about the creator, the contributors, the source. The metadata definitions are provided as components.
Therefore, any entity type can add metadata by adding the metadata component. It can be combined with the file
component for example.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-plugin-metadata/rust.yml">](https://github.com/inexorgame/inexor-rgf-plugin-metadata/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-metadata">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-metadata">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-metadata">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-metadata)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-metadata">](https://github.com/inexorgame/inexor-rgf-plugin-metadata/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Supported data formats

* [Dublin Core](https://en.wikipedia.org/wiki/Dublin_Core) - Documents
  * https://www.dublincore.org/specifications/dublin-core/dces/ 
* [EXIF](https://en.wikipedia.org/wiki/Exif) - Photos
* [ID3](https://en.wikipedia.org/wiki/ID3) - MP3
* ICY - IceCast Streaming Server

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                                |             |                                                                   |
|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-metadata/main/docs/images/icon_CLion.svg"></a> | JetBrains   | Special thanks to JetBrains for providing us with CLion licenses! |
