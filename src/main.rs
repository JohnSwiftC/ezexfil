use std::net::{TcpListener, TcpStream};
use std::error::Error as StdError;
use std::io::ErrorKind;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::io::BufReader;
use std::io::Write;

fn main() -> Result<(), Box<dyn StdError + Send + Sync + 'static>> {
    // Create a socket and listen
    
    let listener = TcpListener::bind("0.0.0.0:80")?;
    
    let message_stack: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    
    

    for stream in listener.incoming() {
        
    }

    Ok(())
}

fn make_log_file(name: &str) -> Result<File, Box<dyn StdError + Send + Sync + 'static>> {
    let mut log_iter = 1;
    loop {
        match File::create_new(format!("{}-{}.txt", name, log_iter)) {
            Ok(f) => {
                return Ok(f);
            },
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    log_iter += 1;
                    continue;
                } else {
                    return Err(Box::new(e));
                }
            },
        } 
    }
}

// Doesn't need a return type, will just panic the thread.
fn handle_connection(stream: TcpStream, message_stack: Arc<Mutex<Vec<String>>>) {
    let buf_reader = BufReader::new(&stream);


}

fn handle_message_stack(message_stack: Arc<Mutex<Vec<String>>>, mut log_file: File) -> Result<(), Box<dyn StdError + Send + Sync + 'static>> {

    loop {

        // Message stack unlocked. We don't want to change the Arc we already have as an input to
        // the function.
        let mut msu = message_stack.lock().expect("message_stack lock failure.");

        let message = msu.pop();
        // Rare manual drop. Unlocks the mutex for other threads to use.
        drop(msu);

        let message = match message {
            Some(m) => m,
            None => continue,
        };

        log_file.write_all(message.as_bytes())?;
    
    }

    Ok(())
}
