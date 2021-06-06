use std::fmt::{Display, Formatter};

struct Frame {
    width: u32,
    height: u32
}

enum VertDir {
    UP,
    DOWN
}

enum HorizDir {
    LEFT,
    RIGHT
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    hor_dir: HorizDir
}

struct Game {
    frame: Frame,
    ball: Ball
}

impl Game {
  fn new() -> Game {
      Game {
      frame: Frame {
        width: 60,
        height: 24
      },
      ball:Ball {
        x: 2,
        y: 4,
        vert_dir: VertDir::UP,
        hor_dir: HorizDir::RIGHT
      }        
    }
  }
  fn step (&mut self) {
    self.ball.bounce(&self.frame);
    self.ball.mv();
  }
}

impl Ball {
  fn bounce (&mut self, frame: &Frame) {
    if self.x==0 {
      self.hor_dir=HorizDir::RIGHT;
    }
    if self.x==frame.width-1 {
      self.hor_dir=HorizDir::LEFT;
    }
    if self.y==0 {
      self.vert_dir = VertDir::DOWN;
    }
    if self.y==frame.height-1 {
      self.vert_dir = VertDir::UP;
    }
  }

  fn mv (&mut self) {
    match self.vert_dir {
      VertDir::UP => self.y -= 1,
      VertDir::DOWN => self.y += 1,
    }
    match self.hor_dir {
      HorizDir::LEFT => self.x -= 1,
      HorizDir::RIGHT => self.x += 1,
    }
  }
}

impl Display for Game {
  fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {

    let top_bottom =  |fmt: &mut Formatter| {
      //print to border
      write!(fmt, "+");
      for _ in 0..self.frame.width {
        write!(fmt, "-");
      }
      write!(fmt, "+\n")
    };

    let _ = top_bottom(fmt);

    for row in 0..self.frame.height {
      write!(fmt,"!");
      for column in 0..self.frame.width {
        let c = if self.ball.x==column && self.ball.y==row {
          "*" 
        } else {
            " "
        };
        write!(fmt,"{}",c);
      }
      write!(fmt,"!\n");
    }

    top_bottom(fmt)
  }
}

fn main() {
    let mut game = Game::new();
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
      println!("{}", game);
      game.step();
      std::thread::sleep(sleep_duration);
    }
}
