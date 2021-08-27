use perseus_actix_web::{configurer, Options};
use actix_web::{HttpServer, App};
use futures::executor::block_on;
use app::{get_templates_map, get_config_manager};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // So we don't have to define a different `FsConfigManager` just for the server, we shift the execution context to the same level as everything else
    // The server has to be a separate crate because otherwise the dependencies don't work with WASM bundling
    env::set_current_dir("../").unwrap();

    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>();
    if let Ok(port) = port {
        HttpServer::new(|| {
            App::new()
            	.configure(
	    			block_on(configurer(
	    				Options {
	    					index: "../index.html".to_string(), // The user must define their own `index.html` file
	    					js_bundle: "dist/pkg/bundle.js".to_string(),
                            // Our crate has the same name, so this will be predictable
	    					wasm_bundle: "dist/pkg/perseus_cli_builder_bg.wasm".to_string(),
	    					templates_map: get_templates_map()
	    				},
	    				get_config_manager()
	    			))
	    		)
        })
        	.bind((host, port))?
        	.run()
        	.await
    } else {
        eprintln!("Port must be a number.");
        Ok(())
    }

}