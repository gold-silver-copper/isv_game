use bevy::{input::keyboard::Key, prelude::*, render::view::visibility};

use bevy_egui::{
    egui::{self, Frame},
    EguiContexts, EguiPlugin,
};
use crate::egui::{FontDefinitions,FontData,FontFamily};
use ratatui::{
    layout::Rect,
    prelude::{Line, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};
use egui_ratatui::RataguiBackend;
use slov_common::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BevyTerminal<RataguiBackend>>()
        //Initialize the ratatui terminal
        .init_resource::<Masterik>()
        .init_resource::<UIState>()
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(PreUpdate, local_world_process)
        .add_systems(PostUpdate, keyboard_input_system)
        .add_systems(Startup, create_local_account)
        .add_systems(PostStartup, set_custom_font)
        .run();
}
// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    mut masterok: ResMut<Masterik>,
    ui_status: Res<UIState>,
) {
    draw_ascii_game(
        &mut termres.terminal_game,
        &masterok.client_world,
        &masterok.player_entity_id,
    );

    draw_ascii_info(&mut termres.terminal_info, &masterok);

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(contexts.ctx_mut(), |ui| {
            let av_height = ui.available_height().clamp(100., 1500.);
            let av_width = ui.available_width().clamp(100., 1500.);

            egui::SidePanel::right("containeeee")
                .min_width(260.)
                .max_width(260.)
                .frame(Frame::none())
                .show_inside(ui, |ui| {
                    let mut menu_panel_string = String::new();
                    if ui_status.menu_open == MenuOpen::Take {
                        draw_take_menu(&mut termres.terminal_menu, &mut masterok);
                        menu_panel_string = "take".into();
                    } else if ui_status.menu_open == MenuOpen::Drop {
                        draw_drop_menu(&mut termres.terminal_menu, &mut masterok);
                        menu_panel_string = "drop".into();
                    } else if ui_status.menu_open == MenuOpen::Attack {
                        draw_attack_menu(&mut termres.terminal_menu, &mut masterok);
                        menu_panel_string = "attack".into();
                    }
                    if menu_panel_string != "" {
                        egui::TopBottomPanel::top(menu_panel_string)
                            .min_height(200.)
                            .max_height(200.)
                            .frame(Frame::none())
                            .show_inside(ui, |ui| {
                                ui.add(termres.terminal_menu.backend_mut());
                            });
                    }

                    egui::TopBottomPanel::bottom("info")
                        .min_height(ui.available_height().clamp(100., 1500.))
                        .max_height(ui.available_height().clamp(100., 1500.))
                        .frame(Frame::none())
                        .show_inside(ui, |ui| {
                            ui.add(termres.terminal_info.backend_mut());
                        });
                });

            let remain_height = ui.available_height().clamp(100., 1500.);
            egui::TopBottomPanel::top("game")
                .min_height(remain_height)
                .max_height(remain_height)
                .frame(Frame::none())
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_game.backend_mut());
                });
        });
}

fn draw_ascii_game(
    terminal: &mut Terminal<RataguiBackend>,
    client_world: &MyWorld,
    client_id: &EntityID,
) {
    terminal
        .draw(|frame| {
            let area = frame.size();
            let client_render =
                client_world.create_client_render_packet_for_entity(client_id, &area);
            let client_graphics = client_render.spans_to_render;
            let mut render_lines = Vec::new();
            let needed_height = area.height as i16;

            if client_graphics.len() > 0 {
                for y in (0..needed_height) {
                    let myspanvec: Vec<_> = client_graphics[y as usize]
                        .iter()
                        .map(|x| Span::from(&x.0).fg(x.1).bg(x.2))
                        .collect();

                    let myline = Line::from(myspanvec);

                    render_lines.push(myline);
                }
            }

            //neccesary beccause drawing is from the top
            render_lines.reverse();
            frame.render_widget(
                Paragraph::new(Text::from(render_lines))
                    .on_black()
                    .block(Block::new().title("Igra").borders(Borders::ALL)),
                area,
            );
        })
        .expect("epic fail");
}

fn draw_ascii_info(terminal: &mut Terminal<RataguiBackend>, masterok: &Masterik) {
    let player_data_copy = masterok
        .client_world
        .entity_map
        .get(&masterok.player_entity_id)
        .unwrap_or(&EntityType::None)
        .clone();
    if let EntityType::Human(player_data_into) = player_data_copy {
        let local_player_loc = masterok
            .client_world
            .ent_loc_index
            .get(&masterok.player_entity_id)
            .unwrap_or(&(0, 0));

        let visible_ents = masterok
            .client_world
            .get_visible_ents_from_ent(&masterok.player_entity_id);

        let statiki = &player_data_into.stats;
        let healthik = &player_data_into.cur_health;
        let nameik = &player_data_into.name;
        let veci = &player_data_into.equipment;
        let mut player_inv = player_data_into.inventory.clone();

        let name_string = format! {"{}",nameik.name};
        let health_string =
            format! {"Zdråvje: {}  Dyhańje {}",healthik.health , healthik.stamina_air };
        let stats_string = format! {"Sila: {}  Bystrost́ {}  Råzum: {} ",statiki.sila , statiki.bystrost , statiki.razum };
        let mut wep_string = String::new();
        let mut local_items = String::new();
        let mut inventory_string = String::new();

        for funnumb in 1..9 {
            let ctotik = player_inv.pop().unwrap_or(ItemType::None);
            let ctotik_string = format! {" {},",ctotik.minimal_string().to_lowercase()};

            if (ctotik == ItemType::None) && (funnumb == 1) {
                inventory_string.push_str(" ničto...");
            }

            if ctotik != ItemType::None {
                inventory_string.push_str(&ctotik_string);
            }
        }

        if let Some(mel_wep) = &veci.melee_weapon {
            let mel_str = format! {"V rųkah {}", mel_wep.minimal_string().to_lowercase() };
            wep_string.push_str(&mel_str);
        } else {
            wep_string.push_str("V rųkah ničto");
        }

        if let Some(ran_wep) = &veci.ranged_weapon {
            let ran_str =
                format! {", za daleky boj je {}", ran_wep.minimal_string().to_lowercase() };
            wep_string.push_str(&ran_str);
        }

        if let Some(current_voxel) = masterok.client_world.get_voxel_at(&local_player_loc) {
            let floor_string = format! {"{}",&current_voxel.floor};

            let items = masterok.client_world.get_items_at_point(&local_player_loc);

            let funny_string = format! {" {},",floor_string.to_lowercase()};
            local_items.push_str(&funny_string);

            for itemik in items {
                local_items.push_str(&format!(" {},", itemik.1.minimal_string().to_lowercase()));
            }
        }

        let mut visibility_string = String::from("");

        if visible_ents.len() > 0 {
            for eid in visible_ents {
                let etik = masterok
                    .client_world
                    .entity_map
                    .get(&eid)
                    .unwrap_or(&EntityType::None);
                if etik != &EntityType::None {
                    let stringik =
                        format! {" {},", ISV::acc_sg( &etik.minimal_string().to_lowercase()) };
                    visibility_string.push_str(&stringik);
                }
            }
        } else {
            visibility_string.push_str(" ničego...");
        }

        //  let stats_string = format!{"Sila: {}  Bystrost́ {}  Råzum: {} ",veci.melee_weapon , veci.ranged_weapon};
        //ty vidisz
        //pod toboj
        //u tebja jest v rukah nozz . Dla daljnogo boja imajesz luk derevny s zlotymi strelami.  Nedavno osnovany objekty je

        let mut messages_clone = masterok.messages.clone();
        messages_clone.reverse();

        let mut messages_to_show = Vec::new();
        messages_to_show.push(Line::from(health_string));
        messages_to_show.push(Line::from(stats_string));
        messages_to_show.push(Line::from(""));
        messages_to_show.push(Line::from(wep_string));
        messages_to_show.push(Line::from("Věči...."));

        messages_to_show.push(Line::from(inventory_string));

        messages_to_show.push(Line::from("Pod tobojų... "));
        messages_to_show.push(Line::from(local_items));
        messages_to_show.push(Line::from("Ty vidiš.... "));

        messages_to_show.push(Line::from(visibility_string));

        messages_to_show.push(Line::from(""));

        for massage in messages_clone {
            messages_to_show.push(Line::from(massage));
        }

        terminal
            .draw(|frame| {
                let area = frame.size();

                //neccesary beccause drawing is from the top

                frame.render_widget(
                    Paragraph::new(messages_to_show)
                        .on_black()
                        .block(Block::new().title(name_string).borders(Borders::ALL)),
                    area,
                );
            })
            .expect("epic fail");
    }
}

fn draw_attack_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &mut Masterik) {
    let mut items = masterok
        .client_world
        .get_visible_ents_from_ent(&masterok.player_entity_id);
  
    let chosen_item = items.get(&masterok.list_cursor_index % items.len()).unwrap_or(&0);
    masterok.targeted_ent_id = chosen_item.clone();
    let mut listitemvec = Vec::new();

    let ciotik = masterok
        .client_world
        .entity_map
        .get(&chosen_item)
        .unwrap_or(&EntityType::None);
    let target_string = format!("currently targeting {}", ciotik.minimal_string());

    listitemvec.push(Line::from("Select further / closer with I / K"));
    listitemvec.push(Line::from(target_string));

    terminal
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                List::new(listitemvec).on_gray().block(
                    Block::new()
                        .title("press number to choose item to pick up")
                        .blue()
                        .borders(Borders::ALL),
                ),
                area,
            );
        })
        .expect("epic fail");
}

fn draw_take_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &mut Masterik) {
    let ent_loc = masterok
        .client_world
        .ent_loc_index
        .get(&masterok.player_entity_id)
        .unwrap_or(&(0, 0));
    let mut items = masterok.client_world.get_items_at_point(ent_loc);
    let mut listitemvec = Vec::new();

    for numb in 1..9 {
        let meowmeow = items.pop();
        let meownyaa: (u64, ItemType) = meowmeow.unwrap_or((0, ItemType::None));

        if meownyaa.1 != ItemType::None {
            masterok
                .button_entityid_map
                .insert(numb.clone(), meownyaa.0.clone());

            let item_str = format!("{}    {}", numb, &meownyaa.1.minimal_string());
            let litem: ListItem = item_str.into();
            listitemvec.push(litem);
        }
    }

    terminal
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                List::new(listitemvec).on_gray().block(
                    Block::new()
                        .title("press number to choose item to pick up")
                        .blue()
                        .borders(Borders::ALL),
                ),
                area,
            );
        })
        .expect("epic fail");
}

fn draw_drop_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &mut Masterik) {
    let mut entik = masterok
        .client_world
        .entity_map
        .get(&masterok.player_entity_id)
        .unwrap_or(&EntityType::None);
    let mut items = Vec::new();
    let mut listitemvec = Vec::new();

    match entik {
        EntityType::Human(igrok) => {
            items = igrok.inventory.clone();
        }
        _ => (),
    }

    for numb in 1..9 {
        let itimik = items.pop().unwrap_or(ItemType::None);
        if itimik != ItemType::None {
            let item_str = format!("{}    {}", numb, &itimik.minimal_string());
            let litem: ListItem = item_str.into();
            listitemvec.push(litem);

            masterok.button_itemstruct_map.insert(numb.clone(), itimik);
        }
    }

    terminal
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                List::new(listitemvec).on_gray().block(
                    Block::new()
                        .title("nozmi nomer vybrati ot czego izbaviti se")
                        .blue()
                        .borders(Borders::ALL),
                ),
                area,
            );
        })
        .expect("epic fail");
}

// Create resource to hold the ratatui terminal
#[derive(Resource)]
struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    terminal_game: Terminal<RataguiBackend>,
    terminal_info: Terminal<RataguiBackend>,
    terminal_menu: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let mut backend1 = RataguiBackend::new(20, 20);
        backend1.set_font_size(20);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        let mut backend2 = RataguiBackend::new(20, 20);
        backend2.set_font_size(12);
        let mut terminal2 = Terminal::new(backend2).unwrap();
        let mut backend3 = RataguiBackend::new(20, 20);
        backend3.set_font_size(12);
        let mut terminal3 = Terminal::new(backend3).unwrap();
        BevyTerminal {
            terminal_game: terminal1,
            terminal_info: terminal2,
            terminal_menu: terminal3,
        }
    }
}

#[derive(Resource)]
struct Masterik {
    player_entity_id: EntityID,
    player_account_id: AccountID,

    messages: Vec<String>,
    client_world: MyWorld,
    is_logged_in: bool,
    button_entityid_map: HashMap<ItemKey, EntityID>,
    button_itemstruct_map: HashMap<ItemKey, ItemType>,
    list_cursor_index: usize,
    targeted_ent_id: EntityID,
}

impl Masterik {
    pub fn refresh_menus(&mut self) {
        self.list_cursor_index = 0;
        self.targeted_ent_id = 0;
        self.button_entityid_map.drain();
        self.button_itemstruct_map.drain();
    }
}

impl Default for Masterik {
    fn default() -> Self {
        Self {
            player_entity_id: 0,
            player_account_id: 0,

            messages: Vec::new(),
            client_world: MyWorld::new_test(),
            is_logged_in: false,
            list_cursor_index: 0,
            targeted_ent_id: 0,
            button_entityid_map: HashMap::new(),
            button_itemstruct_map: HashMap::new(),
        }
    }
}

#[derive(PartialEq)]
pub enum MenuOpen {
    None,
    Take,
    Drop,
    Inventory,
    Stats,
    CursorInteract,
    PlayerLocationInteract,
    Attack,
}

#[derive(Resource)]
struct UIState {
    menu_open: MenuOpen,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            menu_open: MenuOpen::None,
        }
    }
}

fn local_world_process(mut masterok: ResMut<Masterik>) {
    masterok.client_world.interpret_and_execute();
    let boop = masterok
        .client_world
        .create_game_data_packet_for_entity(&masterok.player_entity_id);

    if let Some(meow) = boop {
        //generate text messages from these action packets, then push them to the player message viewer
        let packet_actions = meow.action_info;

        for act in packet_actions {
            let zzz = masterok
                .client_world
                .generate_isv_message(&act, &masterok.player_entity_id);
            masterok.messages.push(zzz);
        }

        //TODO HANDLE ACTION MESSAGES

        //      println!("parsed - {:#?}", packet_actions);
        //    println!("current loc is {:#?}", lppos);
    }
}

fn create_local_account(mut masterok: ResMut<Masterik>) {
    let local_info = masterok.client_world.make_account();
    masterok.player_account_id = local_info.0;
    masterok.player_entity_id = local_info.1;
}



fn set_custom_font(mut contexts: EguiContexts) {


    let mut fonts = FontDefinitions::default();

// Install my own font (maybe supporting non-latin characters):
fonts.font_data.insert("my_font".to_owned(),
   FontData::from_static(include_bytes!("../../assets/fonts/hiero_mono.ttf"))); // .ttf and .otf supported

// Put my font first (highest priority):
fonts.families.get_mut(&FontFamily::Proportional).unwrap()
    .insert(0, "my_font".to_owned());

// Put my font as last fallback for monospace:
fonts.families.get_mut(&FontFamily::Monospace).unwrap()
.insert(0, "my_font".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}



fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut masterok: ResMut<Masterik>,
    mut ui_state: ResMut<UIState>,
) {
    let char_up = input.any_pressed([KeyCode::KeyW]);
    let char_down = input.any_pressed([KeyCode::KeyS]);
    let char_left = input.any_pressed([KeyCode::KeyA]);
    let char_right = input.any_pressed([KeyCode::KeyD]);

    let cursor_up = input.any_just_pressed([KeyCode::KeyI]);
    let cursor_down = input.any_just_pressed([KeyCode::KeyK]);
    let cursor_left = input.any_just_pressed([KeyCode::KeyJ]);
    let cursor_right = input.any_just_pressed([KeyCode::KeyL]);

    let char_attack = input.any_just_pressed([KeyCode::KeyY]); // jęti (jme) / vzeti
    let char_take = input.any_just_pressed([KeyCode::KeyG]); // jęti (jme) / vzeti metnuti  imej target range do ktorogo mozno metati dla praktiki zeby povysati skil be ubijstva
    let char_drop = input.any_just_pressed([KeyCode::KeyB]); //izbaviti se
    let char_help = input.any_just_pressed([KeyCode::KeyT]); //pokazati pomoc ?

    let char_one = input.any_just_pressed([KeyCode::Digit1]);
    let char_two = input.any_just_pressed([KeyCode::Digit2]);
    let char_three = input.any_just_pressed([KeyCode::Digit3]);
    let char_four = input.any_just_pressed([KeyCode::Digit4]);
    let char_five = input.any_just_pressed([KeyCode::Digit5]);
    let char_six = input.any_just_pressed([KeyCode::Digit6]);
    let char_seven = input.any_just_pressed([KeyCode::Digit7]);
    let char_eight = input.any_just_pressed([KeyCode::Digit8]);
    let char_nine = input.any_just_pressed([KeyCode::Digit9]);
    let char_zero = input.any_just_pressed([KeyCode::Digit0]);

    let char_backspace = input.any_pressed([KeyCode::Backspace, KeyCode::Delete]);
    let char_quit = input.any_just_pressed([KeyCode::KeyQ]);

    let mut client_action = ActionType::Wait;
    let client_id = masterok.player_account_id.clone();

    if char_up {
        client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::North));
    }
    if char_down {
        client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::South));
    }
    if char_left {
        client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::West));
    }
    if char_right {
        client_action = ActionType::Go(LocativeID::Cardinal(CardinalDirection::East));
    }

    if ui_state.menu_open == MenuOpen::None {
        if char_take {
            ui_state.menu_open = MenuOpen::Take;
        }
        if char_drop {
            ui_state.menu_open = MenuOpen::Drop;
        }
        if char_attack {
            ui_state.menu_open = MenuOpen::Attack;
        }
    } else if ui_state.menu_open == MenuOpen::Take {
        if char_take {
            masterok.refresh_menus();
            ui_state.menu_open = MenuOpen::None;
        }

        if char_one {
            client_action =
                ActionType::Take(masterok.button_entityid_map.get(&1).unwrap_or(&0).clone());
        }
    } else if ui_state.menu_open == MenuOpen::Attack {
        if char_attack {
            masterok.refresh_menus();
            ui_state.menu_open = MenuOpen::None;
        }

        if cursor_down {
            masterok.list_cursor_index += 1;
        }
        if cursor_up {
            if masterok.list_cursor_index > 0 {
                masterok.list_cursor_index -= 1;
            }
        }

        if cursor_right {
            client_action =
                ActionType::MeleeAttack(masterok.targeted_ent_id);
        }
    } else if ui_state.menu_open == MenuOpen::Drop {
        if char_drop {
            masterok.refresh_menus();
            ui_state.menu_open = MenuOpen::None;
        }

        if char_one {
            client_action = ActionType::Drop(
                masterok
                    .button_itemstruct_map
                    .get(&1)
                    .unwrap_or(&ItemType::None)
                    .clone(),
            );
        }
    }

    if char_quit {
        panic!("BYE");
    }
    if client_action != ActionType::Wait {
        masterok.client_world.receive((client_action, client_id));
        //  println!("{:#?}", masterok.client_world);
    }
}
