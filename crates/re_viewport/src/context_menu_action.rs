//TODO(ab): use list items to make those context menu nice to look at

use crate::{Contents, ViewportBlueprint};
use itertools::Itertools;
use re_log_types::{EntityPath, EntityPathFilter};
use re_space_view::{DataQueryBlueprint, SpaceViewBlueprint};
use re_viewer_context::{ContainerId, Item, SpaceViewClassIdentifier, ViewerContext};

trait Action {
    //TODO(ab): should probably return `egui::WidgetText` instead
    fn label(&self, _ctx: &ViewerContext<'_>, _viewport_blueprint: &ViewportBlueprint) -> String {
        String::new()
    }

    fn run(&self, _ctx: &ViewerContext<'_>, _viewport_blueprint: &ViewportBlueprint) {}

    /// run from inside of [`egui::Response.context_menu()`]
    fn ui(
        &self,
        ctx: &ViewerContext<'_>,
        viewport_blueprint: &ViewportBlueprint,
        ui: &mut egui::Ui,
    ) -> egui::Response {
        let label = self.label(ctx, viewport_blueprint);
        let response = ui.button(label);
        if response.clicked() {
            self.run(ctx, viewport_blueprint);
        }
        response
    }
}

//TODO(ab): this function must become much more complex and handle all cases of homogeneous and heterogeneous multi
//          selections
fn actions_for_item(ctx: &ViewerContext<'_>, item: &Item) -> Vec<Box<dyn Action>> {
    match item {
        Item::Container(container_id) => vec![
            VisibilityToggleAction::new(Contents::Container(container_id.clone())),
            Separator::new(),
            ActionSubMenu::new(
                "Add Container",
                [
                    AddContainerAction::new(container_id.clone(), egui_tiles::ContainerKind::Tabs),
                    AddContainerAction::new(
                        container_id.clone(),
                        egui_tiles::ContainerKind::Horizontal,
                    ),
                    AddContainerAction::new(
                        container_id.clone(),
                        egui_tiles::ContainerKind::Vertical,
                    ),
                    AddContainerAction::new(container_id.clone(), egui_tiles::ContainerKind::Grid),
                ],
            ),
            ActionSubMenu::new(
                "Add Space View",
                ctx.space_view_class_registry
                    .iter_registry()
                    .sorted_by_key(|entry| entry.class.display_name())
                    .map(|entry| {
                        AddSpaceViewAction::new(container_id.clone(), entry.class.identifier())
                    }),
            ),
        ],
        Item::SpaceView(space_view_id) => vec![VisibilityToggleAction::new(Contents::SpaceView(
            space_view_id.clone(),
        ))],
        Item::StoreId(_) => vec![],
        Item::ComponentPath(_) => vec![],
        Item::InstancePath(_, _) => vec![],
        Item::DataBlueprintGroup(_, _, _) => vec![],
    }
}

pub fn contex_menu_ui_for_item(
    ctx: &ViewerContext<'_>,
    viewport_blueprint: &ViewportBlueprint,
    item: &Item,
    item_response: &egui::Response,
) {
    item_response.context_menu(|ui| {
        let actions = actions_for_item(ctx, item);
        for action in actions {
            let response = action.ui(ctx, viewport_blueprint, ui);
            if response.clicked() {
                ui.close_menu();
            }
        }
    });
}

// ================================================================================================
// Utilities
// ================================================================================================

/// Group actions into a sub-menu
struct ActionSubMenu {
    label: String,
    actions: Vec<Box<dyn Action>>,
}

impl ActionSubMenu {
    fn new(label: &str, actions: impl IntoIterator<Item = Box<dyn Action>>) -> Box<dyn Action> {
        let actions = actions.into_iter().collect();
        Box::new(Self {
            label: label.to_owned(),
            actions,
        })
    }
}

impl Action for ActionSubMenu {
    fn ui(
        &self,
        ctx: &ViewerContext<'_>,
        viewport_blueprint: &ViewportBlueprint,
        ui: &mut egui::Ui,
    ) -> egui::Response {
        let resp = ui
            .menu_button(&self.label, |ui| {
                for action in &self.actions {
                    let response = action.ui(ctx, viewport_blueprint, ui);
                    if response.clicked() {
                        ui.close_menu();
                    }
                }
            })
            .response;

        resp
    }
}

/// Add a separator to the context menu
struct Separator;

impl Separator {
    fn new() -> Box<dyn Action> {
        Box::new(Self)
    }
}

impl Action for Separator {
    fn ui(
        &self,
        _ctx: &ViewerContext<'_>,
        _viewport_blueprint: &ViewportBlueprint,
        ui: &mut egui::Ui,
    ) -> egui::Response {
        ui.separator()
    }
}

// ================================================================================================
// Basic actions
// ================================================================================================

/// Control the visibility of a container or space view
struct VisibilityToggleAction {
    contents: Contents,
}

impl VisibilityToggleAction {
    fn new(contents: Contents) -> Box<dyn Action> {
        Box::new(Self { contents })
    }
}

impl Action for VisibilityToggleAction {
    fn label(&self, _ctx: &ViewerContext<'_>, viewport_blueprint: &ViewportBlueprint) -> String {
        if viewport_blueprint.is_contents_visible(&self.contents) {
            "Hide".to_owned()
        } else {
            "Show".to_owned()
        }
    }

    fn run(&self, ctx: &ViewerContext<'_>, viewport_blueprint: &ViewportBlueprint) {
        viewport_blueprint.set_content_visibility(
            ctx,
            &self.contents,
            !viewport_blueprint.is_contents_visible(&self.contents),
        );
    }
}

// ================================================================================================
// Container actions
// ================================================================================================

struct AddContainerAction {
    target_container: ContainerId,
    container_kind: egui_tiles::ContainerKind,
}

impl AddContainerAction {
    fn new(
        target_container: ContainerId,
        container_kind: egui_tiles::ContainerKind,
    ) -> Box<dyn Action> {
        Box::new(Self {
            target_container,
            container_kind,
        })
    }
}

impl Action for AddContainerAction {
    fn label(&self, _ctx: &ViewerContext<'_>, _viewport_blueprint: &ViewportBlueprint) -> String {
        format!("{:?}", self.container_kind)
    }

    fn run(&self, _ctx: &ViewerContext<'_>, viewport_blueprint: &ViewportBlueprint) {
        viewport_blueprint.add_container(self.container_kind, Some(self.target_container));
    }
}

// ---

struct AddSpaceViewAction {
    target_container: ContainerId,
    space_view_class: SpaceViewClassIdentifier,
}

impl AddSpaceViewAction {
    fn new(
        target_container: ContainerId,
        space_view_class: SpaceViewClassIdentifier,
    ) -> Box<dyn Action> {
        Box::new(Self {
            target_container,
            space_view_class,
        })
    }
}

impl Action for AddSpaceViewAction {
    fn label(&self, ctx: &ViewerContext<'_>, _viewport_blueprint: &ViewportBlueprint) -> String {
        ctx.space_view_class_registry
            .get_class_or_log_error(&self.space_view_class)
            .display_name()
            .to_owned()
    }

    fn run(&self, ctx: &ViewerContext<'_>, viewport_blueprint: &ViewportBlueprint) {
        let space_view = SpaceViewBlueprint::new(
            self.space_view_class,
            &EntityPath::root(),
            DataQueryBlueprint::new(self.space_view_class, EntityPathFilter::default()),
        );

        viewport_blueprint.add_space_views(
            std::iter::once(space_view),
            ctx,
            Some(self.target_container),
        );
        viewport_blueprint.mark_user_interaction(ctx);
    }
}
