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
    //let mut stack = t.nodes.into_iter().chain(t.floating_nodes.into_iter()).collect::<Vec<Node>>();
    let mut stack = t.nodes; stack.extend(t.floating_nodes);
    let mut tmp_stack: Vec<Node> = Vec::new();
    
    while name.is_empty() {
      for n in stack.drain(..) {
        if n.focused { 
          name = n.name.unwrap(); 
          // https://stackoverflow.com/questions/983451
          name = name.replace("\\", "\\\\");
          name = name.replace("\"", "\\\"");
          break;
        };
        tmp_stack.extend(n.nodes); 
        tmp_stack.extend(n.floating_nodes); 
      };
      //stack.extend(tmp_stack.drain(..));
      stack.append(&mut tmp_stack);
    }

    // get focused workspace
    let ws = c.get_workspaces().unwrap().workspaces.iter().find(|b| b.focused).unwrap().num;

    // get free mem
    s.refresh_memory();
    let free_mem_mb = (s.available_memory() / 1024) / 1024;
  
    // get power info
    m.refresh(&mut battery).expect("OK");
    let bpf: f32  = battery.state_of_charge().into();
    //let bps = format!("{:.0}%", (100.0 * bpf));
    let bps = format!("{:.0}", (100.0 * bpf));
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
    let mut i_bat = i_tmp.replace("__#__", (power_state.to_string() + ":" + bps.as_str() + "%").as_str());
    if power_state == 'D' && bps.parse::<i32>().unwrap() <= 20 { i_bat = i_bat.replace("}", r#", "urgent": true }"#); }

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
