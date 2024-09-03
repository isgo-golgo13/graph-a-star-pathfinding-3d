use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;
use rand::Rng;
use serde::{Deserialize, Serialize};

mod pathfinding;

#[derive(Deserialize)]
struct PathRequest {
    start: (usize, usize),
    goal: (usize, usize),
}

#[derive(Serialize)]
struct PathResponse {
    path: Vec<(usize, usize)>,
    matrix: Vec<Vec<u8>>, // Include the matrix in the response for visualization
}

async fn find_path(payload: web::Json<PathRequest>) -> impl Responder {
    let matrix = generate_large_matrix(100, 100); // 100x100 maze
    info!("Matrix: {:?}", matrix); // Log the generated matrix for debugging

    let path = pathfinding::a_star(&matrix, payload.start, payload.goal);
    match path {
        Some(p) => {
            info!("Path: {:?}", p); // Log the found path for debugging
            HttpResponse::Ok().json(PathResponse { path: p, matrix })
        }
        None => {
            info!("No path found"); // Log when no path is found
            HttpResponse::NotFound().body("No path found")
        }
    }
}

fn generate_large_matrix(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![0; cols]; rows];
    let mut rng = rand::thread_rng();

    // Add random walls to the matrix
    for row in 0..rows {
        for col in 0..cols {
            if rng.gen_bool(0.3) {
                // 30% chance of being a wall
                matrix[row][col] = 1;
            }
        }
    }

    // Ensure start and goal positions are not blocked
    matrix[0][0] = 0; // Start
    matrix[rows - 1][cols - 1] = 0; // Goal

    matrix
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize the logger
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/path", web::post().to(find_path))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
