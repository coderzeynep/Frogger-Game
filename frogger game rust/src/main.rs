
use macroquad::prelude::*;
use std::thread;
use std::time::Duration;

//These constants define the core game parameters such as screen size, object sizes, and timing.
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const FROG_SIZE: f32 = 30.0;
const CAR_SIZE: f32 = 40.0;
const LOG_SIZE: f32 = 80.0;
const LANE_HEIGHT: f32 = 50.0;
const HOME_Y_POSITION: f32 = 50.0; // Evin bulunduğu Y koordinatı
const HOME_HEIGHT: f32 = 10.0; // Evin yüksekliği
const HOME_SPACING: f32 = 100.0;
const TIME_LIMIT: f32 = 45.0; // 20 saniye limit


struct Frog {
    x: f32,     // The frog's position on the x-axis (right-left)
    y: f32,     // The frog's position on the Y-axis (up-down)
    floating: bool, // New field to check if frog is floating
    horizontal_movement: f32, // Horizontal movement direction from logs/plants
}

impl Frog {
    fn new() -> Self {    //Creates a frog.
        Self {
            x: SCREEN_WIDTH / 2.0 - 45.0,  // Start near the center horizontally
            y: SCREEN_HEIGHT - LANE_HEIGHT + 100.0,
            floating: false,
            horizontal_movement: 0.0,
        }
    }

    fn update(&mut self, horizontal_movement: f32) {    //Handles movement
        if is_key_pressed(KeyCode::Up) {
            self.y -= LANE_HEIGHT;
        }
        if is_key_pressed(KeyCode::Down) {
            self.y += LANE_HEIGHT;
        }
        if is_key_pressed(KeyCode::Left) {
            self.x -= 50.0;
        }
        if is_key_pressed(KeyCode::Right) {
            self.x += 50.0;
        }

        if self.floating {  // If floating, move the frog horizontally with the log or plant
            self.x += horizontal_movement;
        }     
        
        self.x = self.x.clamp(0.0, SCREEN_WIDTH - FROG_SIZE); // Keep the frog within screen boundaries
        self.y = self.y.clamp(0.0, SCREEN_HEIGHT - FROG_SIZE + 150.0);
    }
    // Draws the frog sprite on the screen at its current position
    fn draw(&self, texture: &Texture2D, scale: f32) {   //Renders the frog.
        draw_texture_ex(*texture, self.x, self.y, WHITE, DrawTextureParams {  // Dereferencing the reference here
            dest_size: Some(Vec2::new(FROG_SIZE * scale, FROG_SIZE * scale)),
            ..Default::default()
        });
    }   
}

struct Car {
    x: f32,   // The car's position on the X-axis (left-right)
    y: f32,   //up-down
    speed: f32,
}

impl Car {   //Creates a new car
    fn new(y: f32, speed: f32) -> Self {
        Self {
            x: rand::gen_range(0.0, SCREEN_WIDTH),
            y,
            speed,
        }
    }

    fn update(&mut self) {   //Moves the car
        self.x += self.speed;
        if self.x > SCREEN_WIDTH {
            self.x = -CAR_SIZE;     // Reset position when it moves off-screen (right to left)
        }
        if self.x + CAR_SIZE < 0.0 {
            self.x = SCREEN_WIDTH; // Reset position when it moves off-screen (left to right)
        }
    }

    fn draw(&self, texture: Texture2D, scale: f32) {   //Draws the car on the screen
        draw_texture_ex(texture, self.x, self.y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(CAR_SIZE * scale, CAR_SIZE * scale)), // Resmi boyutlandırma
            ..Default::default()
        }); 
    }

    fn collides_with(&self, frog: &Frog) -> bool {   //Checks collision with the frog and cars
        self.x < frog.x + FROG_SIZE
            && self.x + CAR_SIZE > frog.x
            && self.y < frog.y + FROG_SIZE
            && self.y + CAR_SIZE > frog.y
    }
}

struct Log {   //the log's position and the speed
    x: f32,
    y: f32,
    speed: f32,
}

impl Log {     //Creating a New Log
    fn new(y: f32, speed: f32) -> Self {
        Self {
            x: rand::gen_range(0.0, SCREEN_WIDTH),
            y,
            speed,
        }
    }

    fn update(&mut self) {   // Moves the log across the screen and resets its position if it goes off-screen
        self.x += self.speed;
        if self.x > SCREEN_WIDTH {
            self.x = -LOG_SIZE;    // If the log moves off-screen, reset to the left side
        }
        if self.x + LOG_SIZE < 0.0 {
            self.x = SCREEN_WIDTH; // // If the log moves off-screen to the left, reset to the right side
        }
    }

    fn draw(&self, texture: &Texture2D, scale: f32) {   //Draws the log sprite at its position
        draw_texture_ex(*texture, self.x, self.y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(LOG_SIZE * scale, 30.0 * scale)), // Log resmini boyutlandırarak çiziyoruz
            ..Default::default()
        });
    }
    

    fn collides_with(&self, frog: &Frog) -> bool {  //Checks if the log collides with the frog
        self.x < frog.x + FROG_SIZE
            && self.x + LOG_SIZE > frog.x
            && self.y < frog.y + FROG_SIZE
            && self.y + 30.0 > frog.y
    }
}

fn window_conf() -> Conf { //window size
    Conf {
        window_title: "Frogger".to_owned(), // Set window title
        window_width: 800,  // Set initial width
        window_height: 701,  // Set initial height
        fullscreen: false,    // If true, starts in fullscreen mode
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]  //This macro initializes the game window and sets its title to "Frogger"
async fn main() {  //Loads textures (game graphics).
    let frog_texture = load_texture("crazygames_frog.png").await.unwrap(); // Resmi yükle  //If loading fails, .unwrap() prevents crashing by handling errors.
    let car_texture = load_texture("cars.png").await.unwrap(); // cars.png resmini yükle
    let car_texture_left = load_texture("leftcar2.png").await.unwrap(); // Sol yön araba
    let car1_texture = load_texture("othervehicle.png").await.unwrap();
    let truck_texture = load_texture("left_truck.png").await.unwrap();
    let log_texture = load_texture("newlog.png").await.unwrap(); // log.png resmini yükle
    let home_texture = load_texture("2home.png").await.unwrap(); // Load home.png texture
    let purple_texture = load_texture("purple.png").await.unwrap();

    let mut frog = Frog::new();   //mut allows a variable to be modified after its initialization
    let mut cars = vec![Car::new(400.0, 2.0), Car::new(450.0, -2.5), Car::new(500.0, 4.0),Car::new(350.0, -2.1),Car::new(600.0, 1.8),Car::new(550.0, -1.9)];  //Cars are stored in a vec![] (vector), allowing multiple objects.
    let mut logs = vec![Log::new(150.0, 2.0), Log::new(205.0, -1.5), Log::new(258.0, 1.6)];  //Positive speed = moves right, Negative speed = moves left
    let mut score = 0;
    let mut lives = 3;
    let mut time_left = TIME_LIMIT; // Başlangıçta 20 saniye
    

    let home_x_positions = vec![
        10.0, // İlk evin x konumu
        10.0 + HOME_HEIGHT * 7.0 + HOME_SPACING, // İkinci evin x konumu
        10.0 + 2.0 * (HOME_HEIGHT * 7.0 + HOME_SPACING), // Üçüncü evin x konumu
        10.0 + 3.0 * (HOME_HEIGHT * 7.0 + HOME_SPACING), // Dördüncü evin x konumu
    ];

    let home_y_position = 50.0; // Evlerin bulunduğu y pozisyonu


    loop {
        if time_left > 0.0 {
            time_left -= get_frame_time(); // Her frame'de 1 saniyeden az bir şey çıkar
        }

        clear_background(Color::new(0.1843, 0.1137, 0.1255, 1.0));

        draw_text(&format!("Time: {:.1}", time_left), SCREEN_WIDTH - 150.0, 45.0, 30.0, WHITE); //Displays the remaining time (time_left) on the screen.
        
        draw_texture_ex(  //purple area drawing at the below
            purple_texture, 
            0.0, 
            SCREEN_HEIGHT - LANE_HEIGHT * (-1.0), 
            WHITE, 
            DrawTextureParams {
                dest_size: Some(Vec2::new(purple_texture.width() * 0.93, purple_texture.height() * 0.9)), // Adjust scale here
                ..Default::default()
            }
        );
        
        draw_texture_ex(  //the drawing of other purple area at the above
            purple_texture, 
            0.0, 
            SCREEN_HEIGHT - LANE_HEIGHT * 6.0, 
            WHITE, 
            DrawTextureParams {
                dest_size: Some(Vec2::new(purple_texture.width() * 0.93, purple_texture.height() * 0.9)), // Adjust scale here
                ..Default::default()
            }
        );

        draw_texture_ex(home_texture, 0.0, 58.0, WHITE, DrawTextureParams { //Drawing the Top Home Area
            dest_size: Some(Vec2::new(SCREEN_WIDTH, HOME_HEIGHT * 09.0)), // Resizes to fill the screen width and adjust the height
            ..Default::default()
        });
        
        
        
        //This checks if the frog reaches a home area and increases the score if successful.
        for &home_x in home_x_positions.iter() {
            // Kurbağanın y pozisyonu evler için belirlenen y pozisyonunun hizasında olup olmadığını kontrol ediyoruz.
            // Aynı zamanda x pozisyonunun evin x koordinatına uygun olup olmadığını kontrol ediyoruz
            if frog.y <= home_y_position + HOME_HEIGHT && frog.y >= home_y_position &&
                frog.x >= home_x && frog.x <= home_x + HOME_HEIGHT * 7.0 {
    
                // Kurbağa evin içine girdiğinde
                score += 1;
                draw_text("YOU WIN!", SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0, 50.0, YELLOW);
                next_frame().await;
                thread::sleep(Duration::from_secs(0));
                frog = Frog::new(); // Kurbağayı sıfırlıyoruz ve başlangıç pozisyonuna geri gönderiyoruz
            }
        }


        // If time runs out, lose a life and reset time
        if time_left <= 0.0 {
            lives -= 1;
            time_left = TIME_LIMIT; // Reset the timer
            frog = Frog::new(); // Reset frog position
        }
        
        //  Two road lanes are drawn using a loop.
        for i in 0..3 {
            draw_rectangle(0.0, 350.0 + (i as f32 * LANE_HEIGHT * 2.0), SCREEN_WIDTH, LANE_HEIGHT, Color::new(0.1843, 0.1137, 0.1255, 1.0));
        }

        //  line separating water and land
        draw_line(0.0, 150.0, SCREEN_WIDTH, 150.0, 5.0, Color::new(0.0, 0.0, 0.50196, 1.0));

        // Draws a dark blue rectangle to represent the river.
        draw_rectangle(0.0, 150.0, SCREEN_WIDTH, 150.0, Color::new(0.0, 0.0, 0.50196, 1.0)); // Koyu lacivert rengi


        // Drawing Logs and Floating Platforms
        for log in &logs {
            log.draw(&log_texture, 1.7); // Logları resmini kullanarak çiziyoruz
        }
       
        // Update entities
        frog.update(frog.horizontal_movement); // Update frog with current horizontal movement
        for car in &mut cars {
            car.update();
        }
        for log in &mut logs {
            log.update();
        }
        

        // Collision check with cars  //If the frog collides with a car, the player loses one life and the frog is reset.
        for car in &cars {
            if car.collides_with(&frog) {
                lives -= 1;
                frog = Frog::new();
                break;
            }
        }

        // Check if frog reaches home (the safe zone)
        if frog.y <= HOME_Y_POSITION {
            score += 1; // Increase score
            frog.y = SCREEN_HEIGHT - LANE_HEIGHT; // Reset frog to starting position
        }

        // Check if frog is in the water and loses a life, or floats on logs or plants
        if frog.y >= 150.0 && frog.y <= 250.0 {
            let mut on_log_or_plant = false;
            frog.horizontal_movement = 0.0; // Reset horizontal movement for each update

            for log in &logs {
                if log.collides_with(&frog) {
                    on_log_or_plant = true;
                    frog.floating = true;
                    frog.horizontal_movement = log.speed; // Move with the log
                    break;
                }
            }        

            if !on_log_or_plant {
                lives -= 1;
                frog = Frog::new();
            }
        } else {
            frog.floating = false; // Stop floating when not on log/plant
            frog.horizontal_movement = 0.0; // Stop horizontal movement when off water
        }

        // Score check
        
          if frog.y <= 100.0 {
            score += 1;
            draw_text("YOU WIN!", SCREEN_WIDTH / 2.0 - 145.0, SCREEN_HEIGHT / 2.0, 90.0, YELLOW);
            next_frame().await;
            thread::sleep(Duration::from_secs(2));
            frog = Frog::new();
        }

        for car in &mut cars {
            car.update(); // Update car's position
            
            if car.speed >= 2.0 { 
                car.draw(car_texture, 1.2); // Eğer hız 2 veya daha büyükse, normal araba
            } else if car.speed > 0.0 && car.speed < 2.0 { 
                car.draw(car1_texture, 1.3); // Eğer hız 0 ile 2 arasındaysa, car1 çiz
            } else if car.speed > -2.0 && car.speed < 0.0 { 
                car.draw(truck_texture, 1.3);    
            } else { 
                car.draw(car_texture_left, 1.2); // Eğer hız 0 veya negatifse, sol yöne giden araba
            }
        }
        

     
        frog.draw(&frog_texture, 1.6); // `&` işaretiyle referans olarak gönderiyoruz
         // Resmi kullanarak kurbağayı çiz


        // Display score & lives
        draw_text(&format!("Score: {}", score), 16.0, 25.0, 30.0, WHITE);
        draw_text(&format!("Lives: {}", lives), 16.0, 45.0, 30.0, WHITE);

        // Game over check
        if lives == 0 {
            draw_text("GAME OVER", SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0, 50.0, RED);
            next_frame().await;
            thread::sleep(Duration::from_secs(2));
            return;
        }

        next_frame().await;
    }
}
 