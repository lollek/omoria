use crate::commands;

#[no_mangle]
pub extern fn C_commands_show_class_restrictions() {
    commands::show_class_restrictions();
}

