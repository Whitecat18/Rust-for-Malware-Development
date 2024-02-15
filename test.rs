use std::env;

const SHIT_PLACE: &str = "Noo i was outside the World. Let me enter world called Malware Development with uwuwuw Rustiii...";
fn main(){
    let current_path = env::current_exe().expect(SHIT_PLACE);

    let current_dir = current_path.parent().expect("Noo I coudn't find my World ;( ");
    let running_path = current_dir.join("Rust-for-Malware-Development");

    if running_path.exists(){
        println!("Yaayyy. This is my world Where i live with Nerds and Abuse the Binaries ;D");
    } else {
        eprintln!("Noo. This is Shit Place. Let me go back to my Heven Place where i should exists?");
    }
}
