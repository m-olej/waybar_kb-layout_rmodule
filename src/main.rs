// Run 'hyprctl devices' command in through the terminal
// Read output and create a data structure holding all keyboard types registered
// return the active keymap where main == true

use::std::process::Command;
use::std::io::{self, Write};
 
// Helper function
fn type_of<T>(_: &T){
    println!("{}", std::any::type_name::<T>());
}

fn exec() -> Vec<u8>{
    let kb_data = Command::new("hyprctl").arg("devices").output().expect("kb-layout find fail");
   
    println!("Output status: {}", kb_data.status.success());
    io::stdout().write_all(&kb_data.stdout).unwrap();    
    type_of(&kb_data.stdout); 

    kb_data.stdout
}

fn main() {
    exec();    
}
