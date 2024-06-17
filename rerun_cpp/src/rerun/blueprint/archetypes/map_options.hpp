// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/map_options.fbs".

#pragma once

#include "../../blueprint/components/map_provider.hpp"
#include "../../collection.hpp"
#include "../../data_cell.hpp"
#include "../../indicator_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: Configuration for the background of a view.
    struct MapOptions {
        /// Map provider and style to use.
        rerun::blueprint::components::MapProvider provider;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.MapOptionsIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        MapOptions() = default;
        MapOptions(MapOptions&& other) = default;

        explicit MapOptions(rerun::blueprint::components::MapProvider _provider)
            : provider(std::move(_provider)) {}
    };

} // namespace rerun::blueprint::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<blueprint::archetypes::MapOptions> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(
            const blueprint::archetypes::MapOptions& archetype
        );
    };
} // namespace rerun
