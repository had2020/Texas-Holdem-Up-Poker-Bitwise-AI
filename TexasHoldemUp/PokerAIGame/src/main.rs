use TerimalRtdm::*;

fn main() {
    let mut app = App::new();
    clear(&mut app);
    raw_mode(true);
    show_cursor(false);

    loop {
        Text::new().show(
            &mut app,
            "Texas-Holdem-Up_Poker-Bitwise-AI by Hadrian Lazic",
            pos!(0, 0),
        );

        if Key::o().case_sen(false).pressed(&mut app, KeyType::q)
            || Key::o().pressed(&mut app, KeyType::Esc)
        {
            break;
        }

        render(&app);
        collect_presses(&mut app);
    }

    restore_terminal();
    clear(&mut app);
    println!("Bye!");
}
