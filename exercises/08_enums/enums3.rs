struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("(none)"),
        color: (0, 0, 0),
        quit: false,
    };

    let message = Message::Resize {
        width: 1920,
        height: 1080,
    };

    // `match` is an expression, not just a statement: every arm yields a value
    // of the same type, and the whole `match` evaluates to it. Matching on
    // `&message` binds the payloads by reference, so `message` itself is only
    // borrowed here -- that's what lets us hand it to `process` afterwards.
    let label = match &message {
        Message::Resize { width, height } => format!("resize to {width}x{height}"),
        Message::Move(point) => format!("move to ({}, {})", point.x, point.y),
        Message::Echo(text) => format!("echo {text:?}"),
        Message::ChangeColor(r, g, b) => format!("recolor to ({r}, {g}, {b})"),
        Message::Quit => String::from("quit"),
    };

    println!("before:     {}x{}", state.width, state.height);
    println!("processing: {label}");
    state.process(message);
    println!("after:      {}x{}", state.width, state.height);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
