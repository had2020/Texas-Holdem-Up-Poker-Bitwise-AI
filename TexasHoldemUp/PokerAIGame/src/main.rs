use ::TerimalRtdm::*;

fn main() {
    let mut app = App::new();
    let mut shell = MinifbShell::new("Not finished yet", 60, 24, 2);

    Text::new()
        .foreground(Color::BrightCyan)
        .style(Style::Bold)
        .vanish(false)
        .show(
            &mut app,
            "+--------------------------------------------------+",
            pos!(2, 1),
        );
    Text::new()
        .foreground(Color::BrightWhite)
        .vanish(false)
        .show(
            &mut app,
            "MINIFB TUI TEST  [ARROWS: MOVE | Q: EXIT",
            pos!(2, 2),
        );
    Text::new()
        .foreground(Color::BrightCyan)
        .vanish(false)
        .show(
            &mut app,
            "+--------------------------------------------------+",
            pos!(2, 3),
        );

    mov_cur_to(&mut app, pos!(4, 2));

    while shell.is_open() {
        if shell.poll_input(&mut app) {
            if Key::o().pressed(&mut app, KeyType::q) || Key::o().pressed(&mut app, KeyType::Esc) {
                break;
            }
            if Key::o().pressed(&mut app, KeyType::RightArrow) {
                Mov::cur().freefloat().right(&mut app, 1);
            }
            if Key::o().pressed(&mut app, KeyType::LeftArrow) {
                Mov::cur().freefloat().left(&mut app, 1);
            }
            if Key::o().pressed(&mut app, KeyType::UpArrow) {
                Mov::cur().freefloat().up(&mut app, 1);
            }
            if Key::o().pressed(&mut app, KeyType::DownArrow) {
                Mov::cur().freefloat().down(&mut app, 1);
            }
        }

        shell.render(&app);
    }
}
