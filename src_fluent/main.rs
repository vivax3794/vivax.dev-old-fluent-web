use fluent_web_runtime::{forget, render_component};

mod Router;

mod App;
mod Background;

mod TopBar;
mod TopBarLink;

mod Home;
mod Other;

mod Role;

fn main() {
    forget(render_component!(App, "mount"))
}
