use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, routing::post, Router};
use std::{fs::File, io::Write, path::PathBuf};
use tokio::sync::OnceCell;

static TMP_DIR: OnceCell<PathBuf> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    // Set up the /tmp directory once
    TMP_DIR
        .set(PathBuf::from("/tmp"))
        .expect("Failed to set TMP_DIR");

    // Build the application
    let app = Router::new().route("/upload", post(upload_file));

    // Start the server
    hyper::server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Handles the file upload
async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    // Process each field in the multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        // Get the file name if present
        let file_name = field
            .file_name()
            .map(ToString::to_string)
            .unwrap_or("uploaded_file.bin".to_string());
        let file_path = TMP_DIR.get().unwrap().join(file_name);

        // Write the file to /tmp
        if let Err(err) = save_file(&file_path, field).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to save file: {}", err),
            );
        }
    }

    (StatusCode::OK, "File uploaded successfully".to_string())
}

/// Saves the file to the given path
async fn save_file(
    file_path: &PathBuf,
    field: axum::extract::multipart::Field<'_>,
) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    let data = field.bytes().await?;
    file.write_all(&data)?;
    Ok(())
}
