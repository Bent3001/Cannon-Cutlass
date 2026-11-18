mod views;

use std::{env, fs, io, path::Path};

use axum::{Router, body::Bytes, http::header, routing::get};
use tokio::net::TcpListener;

struct Asset {
    name: &'static str,
    content_type: &'static str,
    body: Bytes,
}

fn assets() -> io::Result<[Asset; 4]> {
    let generated = Path::new("target/browser");
    let read_generated = |name| {
        let path = generated.join(name);
        fs::read(&path).map(Bytes::from).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("{}: {error}. Zuerst `just wasm` ausführen.", path.display()),
            )
        })
    };
    Ok([
        Asset {
            name: "style.css",
            content_type: "text/css; charset=utf-8",
            body: Bytes::from_static(include_bytes!("../assets/style.css")),
        },
        Asset {
            name: "app.js",
            content_type: "text/javascript; charset=utf-8",
            body: Bytes::from_static(include_bytes!("../assets/app.js")),
        },
        Asset {
            name: "game.js",
            content_type: "text/javascript; charset=utf-8",
            body: read_generated("game.js")?,
        },
        Asset {
            name: "game_bg.wasm",
            content_type: "application/wasm",
            body: read_generated("game_bg.wasm")?,
        },
    ])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    match args.as_slice() {
        [] => serve().await?,
        [command] if command == "export" => export(Path::new("dist"))?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Aufruf: cannon-cutlass-web [export]",
            )
            .into());
        }
    }
    Ok(())
}

async fn serve() -> io::Result<()> {
    let address = env::var("CANNON_CUTLASS_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let mut app = Router::new()
        .route("/", get(|| async { views::home() }))
        .route("/healthz", get(|| async { "ok" }));
    for Asset {
        name,
        content_type,
        body,
    } in assets()?
    {
        app = app.route(
            &format!("/assets/{name}"),
            get(move || async move { ([(header::CONTENT_TYPE, content_type)], body) }),
        );
    }
    let listener = TcpListener::bind(address).await?;

    println!(
        "Cannon & Cutlass läuft auf http://{}",
        listener.local_addr()?
    );
    axum::serve(listener, app).await
}

fn export(directory: &Path) -> io::Result<()> {
    let files = assets()?;
    let asset_directory = directory.join("assets");
    fs::create_dir_all(&asset_directory)?;
    for asset in files {
        fs::write(asset_directory.join(asset.name), asset.body)?;
    }
    fs::write(directory.join("index.html"), views::home().into_string())?;
    println!("Statische WebApp exportiert nach {}", directory.display());
    Ok(())
}
