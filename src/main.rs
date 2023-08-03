use ansi_escapes;
use std::thread::sleep;
use std::time::Duration;

struct Body {
    x: usize,
    y: usize,
}
fn main() {
    let mut snake: Vec<Body> = Vec::new();
    let mut size = 1;
    snake.push(Body { x: 0, y: 4 });
    loop {
        
        snake.push(Body{x:snake[0].x +1,y:4});
        snake.drain(0..1);
        let head = snake.last().clone();
        let headpos = snake.len().clone();
        if snake[headpos-1].x > 8 {
            snake[headpos-1].x = 0;
        }
        let frame = draw_snake(&snake);
        render(frame);
    }
}

fn render(frame: [[&str; 9]; 9]) {
    
    for x in frame {
        for y in x {
            print!("{}", y);
        }
        print!("\n")
    }
    sleep(Duration::from_millis(200));
    print!("{}",ansi_escapes::CursorUp(9));
}

fn draw_snake(snake: &Vec<Body>) -> [[&'static str; 9]; 9] {
    let mut canvas = [["⬛"; 9]; 9];
    for x in snake {
        canvas[x.x][x.y] = "⬜"
    }
    canvas
}
