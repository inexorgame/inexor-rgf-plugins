# Plugin Base

This plugin provides some essential components, entity types and relation types.

## Components

| Name        | Properties   | DataType | SocketType | Description                                                                                                                             |
|-------------|--------------|----------|------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| named       | name         | string   | None       |                                                                                                                                         |
| describable | description  | string   | None       |                                                                                                                                         |
| flow_2d     | f2dx         | number   | None       |                                                                                                                                         |
|             | f2dy         | number   | None       |                                                                                                                                         |
|             | f2dw         | number   | None       |                                                                                                                                         |
|             | f2dh         | number   | None       |                                                                                                                                         |
| flow_3d     | f3dx         | number   | None       |                                                                                                                                         |
|             | f3dy         | number   | None       |                                                                                                                                         |
|             | f3dz         | number   | None       |                                                                                                                                         |
|             | f3dw         | number   | None       |                                                                                                                                         |
|             | f3dh         | number   | None       |                                                                                                                                         |
|             | f3dd         | number   | None       |                                                                                                                                         |
| licensed    | license      | string   | None       | The SPDX license identifier. See: https://spdx.org/licenses/                                                                            |
|             | attribution  | string   | None       | Title, author, source and license. Best practices for attribution: https://wiki.creativecommons.org/wiki/best_practices_for_attribution |
| versioned   | version      | string   | None       | The version number. Use semantic versioning. See: https://semver.org/                                                                   |

## Entity Types

| Name    | Properties   | DataType | SocketType | Description                                                                                                                             |
|---------|--------------|----------|------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| comment | name         | string   | None       |                                                                                                                                         |
|         | description  | string   | None       |                                                                                                                                         |

## TODO

- [ ] Move `flow_2d`, `flow_3d` and `comment` to `plugin-flow`

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
