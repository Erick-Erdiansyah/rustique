use bevy::prelude::*;

pub fn sample_ui_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Start,
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        width: Val::Px(f32::INFINITY),
        height: Val::Px(100.0),
        ..Node::DEFAULT
    }
}

pub fn text_sample_ui_style() -> Node {
  Node {
      flex_direction: FlexDirection::Row,
      justify_content: JustifyContent::FlexStart,
      align_items: AlignItems::Center,
      margin: UiRect {
          left: Val::Px(15.0),
          top: Val::Px(15.0),
          ..Default::default()
      },
      border: UiRect::all(Val::Px(10.0)),
      width: Val::Px(240.0),
      height: Val::Px(70.0),
      ..Node::DEFAULT
  }
}
