use macroquad::{prelude::*, ui::{hash, root_ui, widgets}};

use crate::{components::core::Core, log, util::language};

struct MenuItem {
  text: String,
  pos: Option<Vec2>,
  on_select: fn(core: &mut Core, br: &mut bool),
}

impl MenuItem {
  fn new(text: String, pos: Option<Vec2>, on_select: fn(core: &mut Core, br: &mut bool)) -> MenuItem {
    MenuItem { text, pos, on_select }
  }
}

static FONT_SIZE: f32 = 24.0;

fn mouse_within_text(text_pos: Vec2, text_dimension: TextDimensions) -> bool {
  let mouse_pos = mouse_position();
  let text_width = text_dimension.width;
  let text_height = text_dimension.height;

  let x = text_pos.x;
  let y = text_pos.y;

  mouse_pos.0 > x && mouse_pos.0 < x + text_width && mouse_pos.1 > y && mouse_pos.1 < y + text_height
}

/// The main menu is made up of 4 (future 5) buttons. New Game, Options, Credits, and Quit.
/// They are drawn on the bottom left of the screen, spaced apart a little bit, stacked on top of each other.
pub async fn draw(core: &mut Core) -> Result<(), std::io::Error> {
  let cont = language::get(core, "MENU_CONTINUE")?;
  let new_game = language::get(core, "MENU_NEW_GAME")?;
  let options = language::get(core, "MENU_OPTIONS")?;
  let credits = language::get(core, "MENU_CREDITS")?;
  let quit = language::get(core, "MENU_QUIT")?;

  let mut br = false;

  let mut menu_items = vec![
    MenuItem::new(quit, None, |_, _| {
      std::process::exit(0);
    }),
    MenuItem::new(credits, None, |_, _| {
      log!("Unimplemented")
    }),
    MenuItem::new(options, None, |_, _| {
      log!("Unimplemented")
    }),
    MenuItem::new(new_game, None, |core, br| {
      core.current_stage = crate::states::Stage::ShipSelect;
      *br = true;
    }),
    MenuItem::new(cont, None, |_, _| {}),
  ];

  loop {
    clear_background(BLACK);

    // Calculate XY positions for each item
    let mut y = screen_height() - FONT_SIZE;
    for item in menu_items.iter_mut() {
      item.pos = Some(vec2(0., y));
      y -= FONT_SIZE;
    }
    
    for item in &menu_items {
      let btn = widgets::Button::new(item.text.clone())
        .position(item.pos.unwrap())
        .size(vec2(200.0, FONT_SIZE))
        .ui(&mut *root_ui());

      if btn {
        (item.on_select)(core, &mut br);
      }
    }

    next_frame().await;

    if br {
      break;
    }
  }

  Ok(())
}