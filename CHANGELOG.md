# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] Mon Dec 18 CET 2023

Extended applicable implementations with unit tests.

### Changed

* [core/dimension_2d.rs](src/core/dimension_2d.rs) - Renamed `to_array` function to `as_array`.
* [core/position_2d.rs](src/core/position_2d.rs) - Renamed `to_array` function to `as_array`.


### Housekeeping

* [components/coord_2d.rs](src/components/coord_2d.rs) - Added unit tests for existing functionality.
* [core/dimension_2d.rs](src/core/dimension_2d.rs) - Added unit tests for existing functionality.
* [core/position_2d.rs](src/core/position_2d.rs) - Added unit tests for existing functionality.
* [plugins/game_state_plugin.rs](src/plugins/game_state_plugin.rs) - Added unit tests for existing functionality.


## [0.1.5] Sun Dec 17 CET 2023

Added the base structures and functionality for the game, as well as the scaffolding for the web version.

### Added

* [input.json](config/input.json) - To provide a configurable and persistable way to define the game's input scheme.
* [window.json](config/window.json) - To provide a configurable and persistable way to define the dimensions and render
data for the game's display window.
* [components module](src/components/mod.rs) - Organizes and groups all components in the game.
* [components/ascii_sprite.rs](src/components/ascii_sprite.rs) - Component marking an entity as renderable sprite
of the game, made up of an ascii symbol, a foreground and background color.
* [components/coord_2d.rs](src/components/coord_2d.rs) - A positional Component describing the location 
of the associated entity in a two dimensional space with its x and y values.
* [components/game_map.rs](src/components/game_map.rs) - A map making up a level of the game, which the player 
can traverse and explore. It is made up of a linear vector of tiles in which the different entities of the reside in.
* [components/player.rs](src/components/player.rs) - Marker Component used to identify respective entity as the
player / main actor of the game. The resulting entity will be controllable by the player, fight monsters, 
end the game when it dies, etc.
* [core module](src/core/mod.rs) - Organizes and groups all core functionality of the game.
* [core/app_state.rs](src/core/app_state.rs) - Defines all states the game can be in, with every state representing
an isolated and distinct logic section in the game's state machine.
* [core/constants.rs](src/core/constants.rs) - Contains all constant values of the game.
* [core/dimension_2d.rs](src/core/dimension_2d.rs) - Describes a two dimensional area defined by a horizontal width 
and a vertical height.
* [core/plugin_provider.rs](src/core/plugin_provider.rs) - Provides conversion functionality between configuration 
structures and and Plugins, e.g., a structure loaded form a local file and/or persistent settings before it is
converted to a Plugin in order to configure the game.
* [core/position_2d.rs](src/core/position_2d.rs) - Describes a position in a two-dimensional state, 
consisting of a x and `y``coordinate.
* [core/tile.rs](src/core/tile.rs) - Defines all possible tiles which can be displayed on the in-game map.
* [core/view.rs](src/core/view.rs) - Abstraction layer to render different UI components for the game.
* [entities module](src/entities/mod.rs) - Exposes functionality to create and manage all entities for the game.
* [entities/player_bundle.rs](src/entities/player_bundle.rs) - Bundle defining the markup of the player entity 
and handling its spawning logic.
* [js module](src/js/mod.rs) - Provides access to and interoperability with all aspects concerning 
the Javascript system.
* [js/local_storage.rs](src/js/local_storage.rs) - Allows the reading and writing of [key => value] based data 
in the local storage of the browser system. This takes the place of the [file_system.rs](src/os/file_system.rs) 
to persist data, when the game is running in the wasm32-unknown-unknown target.
* [os module](src/os/mod.rs) - Provides access and interoperability with all aspects concerning the current
platform's operating system.
* [os/file_system.rs](src/os/file_system.rs) - Provides functions to interface with the file system of the
current platform's OS, this includes all necessary fs operations like reading and writing files.
* [plugins module](src/plugins/mod.rs) - Provides the game bevy game engine with a all plugins and their systems
required to run the game.
* [plugins/bootstrap_plugin.rs](src/plugins/bootstrap_plugin.rs) - Initial entrypoint Plugin of the game.
* [plugins/game_state_plugin.rs](src/plugins/game_state_plugin.rs) - Plugin coupled with the `AppState::Game` state,
which makes up the main gameplay state. In it the user moves the player entity, fights or otherwise interacts
with the game.
* [res module](src/res/mod.rs) - Provides access to all available `bevy::prelude::Resources` for the game.
* [res/config_file.rs](src/res/config_file.rs) - Allows structs which provide configuration data for the game to be
loaded from local files. This allows for a mutable default setup, which can be customized by the user.
* [res/input_config.rs](src/res/input_config.rs) - Serves as a translator between the raw periphery / hardware inputs 
from the user, e.g., keyboard inputs and mouse clicks, to events which can processed by the game in form of InputTypes.
* [res/window_config.rs](src/res/window_config.rs) - A `bevy::prelude::Resource` for configuring and creating the
display Window of the game.

#### Web

* [index.html](web/index.html) - As the main entry point for the game's web version.
* [storage.js](web/bridge/storage.js) - To serve as the actual `Javascript` implementation for
the `wasm_bindgen` definitions of the [local_storage.rs](src/js/local_storage.rs) to persist data in the
web version of the game.

### Changed

* [main.rs](src/main.rs) - By adding logging capabilities for the web version and
updating the app initialization with the `bootstrap_plugin`.

### Removed

* [.cargo/Config.toml] - Because the wasm-server-runner didn't work with the local assets. 
A nodejs local webserver is now used for testing.

### Housekeeping

* Updated documentation
* [.gitignore](.gitignore) - Updated with the definitions for MacOs specific tmp files
* Added copyright file template and IDE configuration for the MIT license
* Added deployment IDE configuration for publishing the web bundle to the local dev server
* Added IDE run configurations for all common tasks
* [copy_resources_to_target.sh](scripts/copy_resources_to_target.sh) - Added script to copy the game's resources to
the respective buidl target, e.g., native and web
* [build_native.sh](scripts/build_native.sh) - Added script to build the native version of the game for all available 
flavours including the copying of resources.
* [build_wasm.sh](scripts/build_wasm.sh) - Added script to build the web version of the game for all available
flavours including the copying of resources. The script also handles the `wasm-bindgen` generation.

## [0.1.0] - Sun Nov 12 CET 2023

Initial commit providing the base setup for the project.

### Added

* .gitignore
* .vscode/launch.json
* .vscode/tasks.json
* CHANGELOG.md
* Cargo.lock
* Cargo.toml
* LICENCE.md
* README.md
* src/main.rs