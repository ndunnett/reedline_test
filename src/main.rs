/// Minimal reedline REPL
fn main() {
    let mut rl = reedline::Reedline::create();
    let prompt = reedline::DefaultPrompt::default();

    loop {
        match rl.read_line(&prompt) {
            Ok(reedline::Signal::Success(buffer)) => {
                if buffer.trim() == "exit" {
                    break;
                }

                println!("Received: {buffer}");
            }
            Ok(reedline::Signal::CtrlC) => continue,
            Ok(reedline::Signal::CtrlD) => break,
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
}
