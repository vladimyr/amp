use models::application::Event;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use view::Terminal;

pub struct InputListener {
    terminal: Arc<Terminal + Sync + Send>,
    events: Sender<Event>,
    killswitch: Receiver<()>
}

impl InputListener {
    /// Spins up a thread that loops forever, waiting on input from the user
    /// and forwarding key presses to the application event channel.
    pub fn start(terminal: Arc<Terminal + Sync + Send>, events: Sender<Event>, killswitch: Receiver<()>) {
        thread::spawn(move || {
            InputListener {
                terminal: terminal,
                events: events,
                killswitch: killswitch
            }.listen();
        });
    }

    fn listen(&mut self) {
        loop {
            if let Some(key) = self.terminal.listen() {
                self.events.send(Event::Key(key)).ok();
            } else if self.killswitch.try_recv().is_ok() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use input::Key;
    use models::application::Event;
    use std::sync::Arc;
    use std::sync::mpsc;
    use super::InputListener;
    use view::terminal::test_terminal::TestTerminal;

    #[test]
    fn start_listens_for_and_sends_key_events_from_terminal() {
        let terminal = Arc::new(TestTerminal::new());
        let (event_tx, event_rx) = mpsc::channel();
        let (_, killswitch_rx) = mpsc::sync_channel(0);
        InputListener::start(terminal.clone(), event_tx, killswitch_rx);
        let event = event_rx.recv().unwrap();

        assert_eq!(event, Event::Key(Key::Char('A')));
    }
}
