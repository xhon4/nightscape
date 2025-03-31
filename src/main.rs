use crossterm::{
    cursor, execute, style::Print, terminal,
    terminal::{size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::{
    io::stdout,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::Duration,
};
use signal_hook::consts::SIGINT;
use signal_hook::flag;

const WIDTH: usize = 80;
const HEIGHT: usize = 20;
const _PHASES: [&str; 8] = ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];
const BRIGHTNESS_LEVELS: [char; 7] = ['.', '•', '*', '✦', '*', '•', '.'];
const FRAME_DELAY: u64 = 50;
const ASCII_MOON_PHASES: [&[&str]; 1] = [
    &[
        "     _.._     ",
        "   .' .-'`    ",
        "  /  /        ",
        "  |  |        ",
        "  \\  \\        ",
        "   '._'-._    ",
        "      ```     ",
    ],
];

fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}

fn render_sky(
    moon_x: usize,
    moon_y: usize,
    _moon_width: usize,
    _moon_height: usize,
    stars: &Vec<(usize, usize, char)>,
) {
    let mut stdout = stdout();
    let moon_phase = ASCII_MOON_PHASES[0];
    for (i, line) in moon_phase.iter().enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(moon_x as u16, (moon_y + i) as u16),
            Print(line)
        )
        .unwrap();
    }
    for &(x, y, brightness) in stars {
        execute!(stdout, cursor::MoveTo(x as u16, y as u16), Print(brightness)).unwrap();
    }
}

fn update_stars(stars: &mut Vec<(usize, usize, char)>) {
    let mut rng = rand::thread_rng();
    for star in stars.iter_mut() {
        let (_, _, ref mut brightness) = *star;
        let current_index = BRIGHTNESS_LEVELS.iter().position(|&c| c == *brightness).unwrap_or(0);
        let new_index = match rng.gen_range(0..=2) {
            0 if current_index > 0 => current_index - 1,
            1 if current_index < BRIGHTNESS_LEVELS.len() - 1 => current_index + 1,
            _ => current_index,
        };
        *brightness = BRIGHTNESS_LEVELS[new_index];
    }
}

fn generate_sky() -> (usize, usize, usize, usize, Vec<(usize, usize, char)>) {
    let mut rng = rand::thread_rng();
    let (term_width, term_height) = size().unwrap_or((WIDTH as u16, HEIGHT as u16));
    let term_width = term_width as usize;
    let term_height = term_height as usize;
    let moon_phase = ASCII_MOON_PHASES[0];
    let _moon_width = moon_phase[0].len();
    let _moon_height = moon_phase.len();
    let moon_x = rng.gen_range(0..=term_width.saturating_sub(_moon_width));
    let max_y = (term_height as f32 * 0.3).ceil() as usize;
    let moon_y = rng.gen_range(0..=max_y.saturating_sub(_moon_height));
    let mut stars = Vec::new();
    while stars.len() < 74 {
        let x = rng.gen_range(0..term_width);
        let y = rng.gen_range(0..term_height);
        if !(moon_y..moon_y + _moon_height).contains(&y)
            || !(moon_x..moon_x + _moon_width).contains(&x)
        {
            stars.push((x, y, BRIGHTNESS_LEVELS[rng.gen_range(0..BRIGHTNESS_LEVELS.len())]));
        }
    }
    (moon_x, moon_y, _moon_width, _moon_height, stars)
}

fn random_event(
    stars: &mut Vec<(usize, usize, char)>,
    term_width: usize,
    term_height: usize,
    moon_x: usize,
    moon_y: usize,
    moon_width: usize,
    moon_height: usize,
) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..1000) { // Aumentamos el rango para reducir la frecuencia de eventos
        0..=4 => turn_off_star(stars, term_width, term_height, moon_x, moon_y, moon_width, moon_height), // Evento raro
        5..=6 => spawn_comet(term_width, term_height), // Cometa sigue siendo poco frecuente
        7 => spawn_ufo(term_width, term_height), // OVNI es extremadamente raro
        _ => {}
    }
}

fn turn_off_star(
    stars: &mut Vec<(usize, usize, char)>,
    term_width: usize,
    term_height: usize,
    moon_x: usize,
    moon_y: usize,
    moon_width: usize,
    moon_height: usize,
) {
    let mut rng = rand::thread_rng();
    if !stars.is_empty() {
        let index = rng.gen_range(0..stars.len());
        if let Some(star) = stars.get_mut(index) {
            // Apagar la estrella actual
            star.2 = ' ';
            
            // Generar nueva posición para la estrella
            let mut new_x;
            let mut new_y;
            loop {
                new_x = rng.gen_range(0..term_width);
                new_y = rng.gen_range(0..term_height);
                // Asegurarse de que no esté dentro del área de la luna
                if !(moon_y..moon_y + moon_height).contains(&new_y) || !(moon_x..moon_x + moon_width).contains(&new_x) {
                    break;
                }
            }
            // Reubicar la estrella con un nuevo brillo
            *star = (new_x, new_y, BRIGHTNESS_LEVELS[rng.gen_range(0..BRIGHTNESS_LEVELS.len())]);
        }
    }
}

fn create_new_star(
    stars: &mut Vec<(usize, usize, char)>,
    term_width: usize,
    term_height: usize,
    moon_x: usize,
    moon_y: usize,
    moon_width: usize,
    moon_height: usize,
) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..term_width);
    let y = rng.gen_range(0..term_height);
    if !(moon_y..moon_y + moon_height).contains(&y) || !(moon_x..moon_x + moon_width).contains(&x) {
        stars.push((x, y, BRIGHTNESS_LEVELS[rng.gen_range(0..BRIGHTNESS_LEVELS.len())]));
    }
}

fn spawn_comet(term_width: usize, term_height: usize) {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();
    let start_x = rng.gen_range(0..term_width);
    let start_y = rng.gen_range(0..term_height);
    execute!(
        stdout,
        cursor::MoveTo(start_x as u16, start_y as u16),
        Print("☄")
    )
    .unwrap();
    thread::sleep(Duration::from_millis(300));
    execute!(
        stdout,
        cursor::MoveTo(start_x as u16, start_y as u16),
        Print(" ")
    )
    .unwrap();
}

fn spawn_ufo(term_width: usize, term_height: usize) {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();
    let ufo_x = rng.gen_range(0..term_width);
    let ufo_y = rng.gen_range(0..term_height / 2);
    execute!(
        stdout,
        cursor::MoveTo(ufo_x as u16, ufo_y as u16),
        Print("🛸")
    )
    .unwrap();
    thread::sleep(Duration::from_millis(500));
    execute!(
        stdout,
        cursor::MoveTo(ufo_x as u16, ufo_y as u16),
        Print(" ")
    )
    .unwrap();
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    flag::register(SIGINT, r).expect("Failed to register SIGINT handler");
    terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All), cursor::Hide).unwrap();
    let (moon_x, moon_y, moon_width, moon_height, mut stars) = generate_sky();
    let (term_width, term_height) = size().unwrap_or((WIDTH as u16, HEIGHT as u16));
    let term_width = term_width as usize;
    let term_height = term_height as usize;
    while running.load(Ordering::SeqCst) {
        clear_screen();
        render_sky(moon_x, moon_y, moon_width, moon_height, &stars);
        update_stars(&mut stars);
        random_event(&mut stars, term_width, term_height, moon_x, moon_y, moon_width, moon_height);
        thread::sleep(Duration::from_millis(FRAME_DELAY));
    }
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, LeaveAlternateScreen, cursor::Show).unwrap();
    println!("Program terminated. See you next time!");
}
