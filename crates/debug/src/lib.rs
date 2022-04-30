use bevy::{
  diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
  ecs::schedule::ShouldRun,
  input::{keyboard::KeyboardInput, ElementState},
  prelude::{CoreStage, EventReader, KeyCode, Plugin, Res, ResMut, SystemSet, SystemStage},
};
use bevy_egui::{
  egui::{self},
  EguiContext, EguiPlugin,
};

fn display_debug_stats(mut egui: ResMut<EguiContext>, diagnostics: Res<Diagnostics>) {
  egui::Window::new("Performance").show(egui.ctx_mut(), |ui| {
    ui.label(format!(
      "Avg. FPS: {:.02}",
      diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .average()
        .unwrap_or_default()
    ));
    ui.label(format!(
      "Total Entity count: {}",
      diagnostics
        .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .unwrap()
        .average()
        .unwrap_or_default()
    ));
  });
}

fn display_debug_ui_criteria(ui_state: Res<DebugUIState>) -> ShouldRun {
  if ui_state.display_debug_info {
    ShouldRun::Yes
  } else {
    ShouldRun::No
  }
}

fn toggle_debug_ui_displays(
  mut inputs: EventReader<KeyboardInput>,
  mut ui_state: ResMut<DebugUIState>,
) {
  for input in inputs.iter() {
    match input.key_code {
      Some(key_code) if key_code == KeyCode::F3 && input.state == ElementState::Pressed => {
        ui_state.display_debug_info = !ui_state.display_debug_info;
      }
      _ => {}
    }
  }
}

pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .add_plugin(EguiPlugin)
      .add_plugin(FrameTimeDiagnosticsPlugin)
      .add_plugin(EntityCountDiagnosticsPlugin)
      .add_stage_after(
        CoreStage::PostUpdate,
        "debug_ui_stage",
        SystemStage::parallel()
          .with_system(toggle_debug_ui_displays)
          .with_system_set(
            SystemSet::new()
              .with_system(display_debug_stats)
              .with_run_criteria(display_debug_ui_criteria),
          ),
      )
      .init_resource::<DebugUIState>();
  }
}

#[derive(Default)]
struct DebugUIState {
  display_debug_info: bool,
}
