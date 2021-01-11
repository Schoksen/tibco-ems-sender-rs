extern crate tibco_ems;
extern crate env_logger;

use log::{error, info, debug};
use tibco_ems::{Destination, DestinationType, TextMessage};

use std::fs::{self, File};
use std::io::prelude::*;
use std::fs::ReadDir;

fn env_vars(variable: &str) -> String {
  match std::env::var(variable) { 
    Ok(var) => {
      info!("using {}: {} ", &variable, &var);
      return var;
    },
    Err(e) => { 
    error!("Environment Variable not found: {}={:?}",variable, e);
    return "env_var_error".to_owned();
   }
  }
}

fn main() {
  env_logger::init();
  info!("starting engine");
  info!("using the following config");

  // env variables
  let url:String = env_vars("ems_url");
  let password:String = env_vars("ems_password");
  let user:String = env_vars("ems_user");
  let input_dest_name:String = env_vars("ems_input_dest_name");
  let input_dest_type:String = env_vars("ems_input_dest_type");
  const CHECK:&str = "env_var_error";
  if url == CHECK || password== CHECK || user == CHECK || input_dest_name == CHECK || input_dest_type == CHECK {
    panic!("Not all required environment variables have been set please see error log!");
  }

  let connection = match tibco_ems::connect(url,user,password) { Ok(var) => var, Err(e) => {
    error!("Unknown EMS Connection Error");
    error!("{:?}", e);
    return (); }};

  // send msg and add testcase response
  let input_destination: Destination = match input_dest_type.as_str().to_lowercase().trim() {
    "topic" => Destination{
          destination_type: DestinationType::Topic,
          destination_name: input_dest_name.clone()}
    ,
    "queue" => Destination{
        destination_type: DestinationType::Queue,
        destination_name: input_dest_name.clone()}
    ,
    _ => {
      error!("ems_input_dest_type needs to be of queue/topic"); return ();
    }
  };

  let session = match connection.session() { Ok(sesh) => sesh, Err(e) => {
      error!("Unknown EMS Session Error");
      error!("{:?}", e); 
      return ();}
    };

  let dir:ReadDir = match fs::read_dir("input") {
    Err(why) => {println!("! {:?}", why.kind()); return ()},
    Ok(paths) => paths
  }; 

  debug!("Input Directory: {:?}", dir);
 
  for file_list in dir {
    match file_list {
      Ok(file_list) => {
      let input_file:String = file_list.path().into_os_string().into_string().unwrap();
      let mut fifi:File = match File::open(&input_file) { Ok(var) => var, Err(e) => {
        error!("Input file could not be opened: \n {} \n {}", &input_file, e); return (); }};
      let mut input_contents = String::new();
      let _buffer = fifi.read_to_string(&mut input_contents);
      debug!("Imported file: {}", &input_file);
      let msg: TextMessage = TextMessage{body: input_contents, header: None};
      match session.send_message(input_destination.clone(), msg.into()) {
        Ok(_) => info!("Sent file: {}", &input_file),
        Err(e) => {
          error!("Message could not be send");
          error!("{:?}", e);}
      };
    },
    Err(e) => {
      error!("Unknown EMS Session Error");
      error!("{:?}", e); 
      return ();}
    }
  }   
}