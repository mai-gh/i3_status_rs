use i3ipc::I3Connection;
use i3ipc::reply::Node;
use sysinfo::{System, SystemExt};
use chrono::{DateTime, Local};
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};

fn main() {

  // initalize for i3
  let mut c = I3Connection::connect().unwrap();

  // initialize for ram
  let mut s = System::new_all();

  // initialize for power
  let m = battery::Manager::new().expect("OK");
  let mut batteries = m.batteries().expect("OK");
  let mut battery = batteries.next().expect("OK").expect("OK");

  // initialize waiting
  let one_sec = Duration::new(1,0);

  // print heading
  let heading = r#"{ "version": 1 }[[],"#;
  print!("{}", heading);

  loop {

    // get focused window
    let t = c.get_tree().unwrap();
    let mut name = "".to_string();
    let mut stack: Vec<Node> = Vec::new();
    let mut tmp_stack: Vec<Node> = Vec::new();
    for n in t.nodes.iter().cloned() {stack.push(n)}        
    for n in t.floating_nodes.iter().cloned() {stack.push(n)}        
    
    while name.is_empty() {
      for n in stack.iter() {
        if n.focused == true { 
            name = n.name.clone().unwrap().to_string(); 
            break;
        };
        for tn in n.nodes.iter().cloned() {tmp_stack.push(tn)}        
        for tn in n.floating_nodes.iter().cloned() {tmp_stack.push(tn)}        
      };
      stack.clear();
      for n in tmp_stack.iter().cloned() {stack.push(n)}
      tmp_stack.clear();
    }

    // get focused workspace
    let ws = c.get_workspaces().unwrap().workspaces.iter().find(|b| b.focused == true).unwrap().num;

    // get free mem
    s.refresh_memory();
    let free_mem_mb = (s.available_memory() / 1024) / 1024;
  
    // get power info
    m.refresh(&mut battery).expect("OK");
    let bpf: f32  = battery.state_of_charge().into();
    //let bps = format!("{:.0}%", (100.0 * bpf));
    let bps = format!("{:.0}%", (100.0 * bpf));
    let b_state = battery.state();
    let power_state = b_state.to_string().to_uppercase().chars().next().unwrap();
  

    // get current time  
    let now: DateTime<Local> = Local::now();
    let time = now.format("%a %b %e %T");
  
  
    // print formatted status line
    let i_tmp = r##"{ "full_text": " __#__ ", "color": "#cccccc" }"##;
    let i_name = i_tmp.replace("__#__", name.as_str());
    let i_ws = i_tmp.replace("__#__", ws.to_string().as_str());
    let i_mem = i_tmp.replace("__#__", (free_mem_mb.to_string() + "M").as_str());
    let i_bat = i_tmp.replace("__#__", (power_state.to_string() + ":" + bps.as_str()).as_str());
    let i_time = i_tmp.replace("__#__", time.to_string().as_str());
    print!("[");
    print!("{},", i_name);
    print!("{},", i_ws);
    print!("{},", i_mem);
    print!("{},", i_bat);
    print!("{}", i_time);
    print!("], ");

    io::stdout().flush().expect("OK");
    sleep(one_sec);
  }
}
