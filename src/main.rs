//! Entry point into the game (hopefully LOL)
use project_abyss::ui::init_screen;

fn main() {
    let mut screen = init_screen();
    screen.clear();
    loop {
        screen.erase();
        // TODO: Move this header into a function
        screen.print("\n ###############################################");
        screen.print("\n #                Project Abyss                #");
        screen.print("\n ###############################################\n\n");
        screen.print("\t         ==> Play <== \n");
        screen.print("\t         ==> Help <== \n");
        screen.print("\nPress q or Q to quit\n\n> ");
        screen.display();
        screen.getch();
        screen.refresh();
    }
}