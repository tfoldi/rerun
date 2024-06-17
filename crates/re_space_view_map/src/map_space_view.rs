use {
    egui::{self, Color32, Context, TextEdit},
    re_entity_db::EntityProperties,
    re_log_types::EntityPath,
    re_space_view::suggest_space_view_for_each_entity,
    re_types::blueprint::components::MapProvider,
    re_types::{
        components::{Color, Radius},
        SpaceViewClassIdentifier, View,
    },
    re_ui::UiExt,
    re_viewer_context::{
        SpaceViewClass, SpaceViewClassLayoutPriority, SpaceViewClassRegistryError, SpaceViewId,
        SpaceViewSpawnHeuristics, SpaceViewState, SpaceViewStateExt as _,
        SpaceViewSystemExecutionError, SpaceViewSystemRegistrator, SystemExecutionOutput,
        ViewQuery, ViewerContext,
    },
    walkers::{Map, MapMemory, Plugin, Tiles, TilesManager},
};

use crate::map_visualizer_system::{MapEntry, MapVisualizerSystem};
use crate::map_windows;

// walkers plugin to visualize points on a Map
pub struct PositionsOnMap {
    positions: Vec<MapEntry>,
}

impl Plugin for PositionsOnMap {
    fn run(
        &mut self,
        _response: &egui::Response,
        painter: egui::Painter,
        projector: &walkers::Projector,
    ) {
        for entry in &self.positions {
            // Position of the point we want to put our shapes.
            let position = entry.position;

            // Project it into the position on the screen.
            let position = projector.project(position).to_pos2();

            // Radius of the circle
            let radius = f32::from(entry.radii.unwrap_or(Radius(10.)));

            // Color of the circle
            let color = entry.color.unwrap_or(Color::new(Color32::RED));

            painter.circle_filled(position, radius, color);
        }
    }
}

#[derive(Default)]
pub struct MapSpaceViewState {
    tiles: Option<Tiles>,
    map_memory: MapMemory,
    selected_provider: MapProvider,
    mapbox_access_token: String,
}

impl MapSpaceViewState {
    // This method ensures that tiles is initialized and returns mutable references to tiles and map_memory.
    pub fn ensure_and_get_mut_refs(
        &mut self,
        ctx: &egui::Context,
    ) -> Result<(&mut Tiles, &mut MapMemory), SpaceViewSystemExecutionError> {
        if self.tiles.is_none() {
            let tiles = get_tile_manager(self.selected_provider, &self.mapbox_access_token, ctx);
            self.tiles = Some(tiles);
        }

        // Now that tiles is guaranteed to be Some, unwrap is safe here.
        let tiles_ref = self
            .tiles
            .as_mut()
            .ok_or(SpaceViewSystemExecutionError::MapTilesError)?;
        Ok((tiles_ref, &mut self.map_memory))
    }
}

impl SpaceViewState for MapSpaceViewState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct MapSpaceView;

type ViewType = re_types::blueprint::views::MapView;

impl SpaceViewClass for MapSpaceView {
    fn identifier() -> SpaceViewClassIdentifier {
        ViewType::identifier()
    }

    fn display_name(&self) -> &'static str {
        "Map"
    }

    fn icon(&self) -> &'static re_ui::Icon {
        &re_ui::icons::SPACE_VIEW_MAP
    }

    fn help_text(&self, _egui_ctx: &egui::Context) -> egui::WidgetText {
        "Map view".into()
    }

    fn on_register(
        &self,
        system_registry: &mut SpaceViewSystemRegistrator<'_>,
    ) -> Result<(), SpaceViewClassRegistryError> {
        system_registry.register_visualizer::<MapVisualizerSystem>()
    }

    fn new_state(&self) -> Box<dyn SpaceViewState> {
        Box::<MapSpaceViewState>::new(MapSpaceViewState {
            tiles: None,
            map_memory: MapMemory::default(),
            selected_provider: MapProvider::default(),
            // TODO(tfoldi): this should come from the app configuration or blueprint
            mapbox_access_token: std::env::var("MAPBOX_ACCESS_TOKEN").unwrap_or_default(),
        })
    }

    fn preferred_tile_aspect_ratio(&self, _state: &dyn SpaceViewState) -> Option<f32> {
        // Prefer a square tile if possible.
        Some(1.0)
    }

    fn layout_priority(&self) -> SpaceViewClassLayoutPriority {
        Default::default()
    }

    fn spawn_heuristics(&self, ctx: &ViewerContext<'_>) -> SpaceViewSpawnHeuristics {
        suggest_space_view_for_each_entity::<MapVisualizerSystem>(ctx, self)
    }

    fn selection_ui(
        &self,
        _ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        _space_origin: &EntityPath,
        _space_view_id: SpaceViewId,
        _root_entity_properties: &mut EntityProperties,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let map_state = state.downcast_mut::<MapSpaceViewState>()?;
        let mut selected = map_state.selected_provider;

        // TODO(tfoldi): seems there is no implementation for combo box in view_property_ui
        // list_item::list_item_scope(ui, "map_selection_ui", |ui| {
        //     view_property_ui::<re_types::blueprint::archetypes::MapOptions>(
        //         ctx,
        //         ui,
        //         space_view_id,
        //         self,
        //         state,
        //     );
        // });

        ui.horizontal(|ui| {
            ui.label("Map provider");
            egui::ComboBox::from_id_source("map_provider")
                .selected_text(format!("{selected:?}"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, MapProvider::OpenStreetMap, "OpenStreetMap");
                    ui.selectable_value(
                        &mut selected,
                        MapProvider::MapboxStreets,
                        "Mapbox Streets (Light)",
                    );
                    ui.selectable_value(
                        &mut selected,
                        MapProvider::MapboxDark,
                        "Mapbox Streets (Dark)",
                    );
                    ui.selectable_value(
                        &mut selected,
                        MapProvider::MapboxSatellite,
                        "Mapbox Satellite",
                    );
                });

            // If the selected provider has changed, reset the tiles.
            if selected != map_state.selected_provider {
                map_state.tiles = None;
                map_state.selected_provider = selected;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Mapbox Access Token").on_hover_text("Access token for Mapbox API. Please refer to the Mapbox documentation for more information.");
            ui.add( TextEdit::singleline(&mut map_state.mapbox_access_token)
                .hint_text("Mapbox Access Token")
                .password(true));
        });

        ui.horizontal(|ui| {
            let mut is_following = map_state.map_memory.detached().is_none();

            if ui
                .re_checkbox(&mut is_following, "Follow positions on map")
                .changed()
            {
                if is_following {
                    map_state.map_memory.follow_my_position();
                } else {
                    // Detach the map from the current position
                    // TODO(tfoldi): should be added to the map_memory API
                }
            }
        });

        Ok(())
    }

    fn ui(
        &self,
        _ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        _root_entity_properties: &EntityProperties,
        _query: &ViewQuery<'_>,
        system_output: SystemExecutionOutput,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let state = state.downcast_mut::<MapSpaceViewState>()?;
        let map_viz_system = system_output.view_systems.get::<MapVisualizerSystem>()?;

        let (tiles, map_memory) = match state.ensure_and_get_mut_refs(ui.ctx()) {
            Ok(refs) => refs,
            Err(e) => return Err(e),
        };

        egui::Frame::default().show(ui, |ui| {
            let some_tiles_manager: Option<&mut dyn TilesManager> = Some(tiles);
            let map_widget = ui.add(
                Map::new(
                    some_tiles_manager,
                    map_memory,
                    map_viz_system
                        .map_entries
                        .first()
                        .unwrap_or(&MapEntry::default())
                        .position,
                )
                .with_plugin(PositionsOnMap {
                    positions: map_viz_system.map_entries.clone(),
                }),
            );

            let map_pos = map_widget.rect;
            map_windows::zoom(ui, &map_pos, map_memory);
            map_windows::acknowledge(ui, &map_pos, tiles.attribution());
        });
        Ok(())
    }
}

fn get_tile_manager(provider: MapProvider, mapbox_access_token: &str, egui_ctx: &Context) -> Tiles {
    match provider {
        MapProvider::OpenStreetMap => Tiles::new(walkers::sources::OpenStreetMap, egui_ctx.clone()),
        MapProvider::MapboxStreets => Tiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Streets,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: false,
            },
            egui_ctx.clone(),
        ),
        MapProvider::MapboxDark => Tiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Dark,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: false,
            },
            egui_ctx.clone(),
        ),
        MapProvider::MapboxSatellite => Tiles::new(
            walkers::sources::Mapbox {
                style: walkers::sources::MapboxStyle::Satellite,
                access_token: mapbox_access_token.to_owned(),
                high_resolution: true,
            },
            egui_ctx.clone(),
        ),
    }
}

re_viewer_context::impl_component_fallback_provider!(MapSpaceView => []);
