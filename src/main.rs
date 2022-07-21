use std::{net::{SocketAddr, TcpListener}, fs::File, path::PathBuf};
use socket2::{Socket, Domain, Type};
use std::io::prelude::*;
use mongodb::{Client, bson::{doc, Document}, options::ClientOptions};
use mongodb::bson;
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    // let _paths = start_checks();
  
    create_server().await;
}


/// It creates a socket, binds it to a port, listens for connections, and then sends the data to the
/// server
async fn create_server() {
    
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    
    let address: SocketAddr = "0.0.0.0:9098".parse().unwrap();
    let address = address.into();
    socket.bind(&address).unwrap();
    socket.listen(128).unwrap();
    let listener: TcpListener = socket.into();
    //Main Event Loop
    loop{
        let mut data_stream = listener.accept().unwrap();
        // Start of message
        println!("----------------------------------------------------------------");
        print!("Recieved a connection from {}\n\n", data_stream.1);

        let mut buffer = String::new();
        data_stream.0.read_to_string(&mut buffer).unwrap();

        print!("Value : {:?}\n\n", buffer);

        //create mongo Data to send

        let chrono_dt: chrono::DateTime<Utc> = Utc::now();
        let bson_dt: bson::DateTime = bson::DateTime::from_chrono(chrono_dt);
        let data = doc! {
            "ValueType" : "Tempature",
            "Value": &buffer,
            "timestamp" : bson_dt
        };
        send_data_to_server(data).await;  
        print!("----------------------------------------------------------------\n\n");
    }
}

/// It checks if the cache directory exists, if it doesn't it creates it. Then it checks if the            -- Possibly remove changed course of design
/// configuration file exists, if it doesn't it creates it. Then it checks if the data file exists, if
/// it doesn't it creates it
// fn start_checks() -> (PathBuf, PathBuf) {
//     let current_directory = std::env::current_dir().unwrap();
//     let cache_dir = current_directory.join("Cache");
//     let json_dir = current_directory.join("Cache\\conf.json");
//     let data_dir = current_directory.join("Cache\\data.val");

//     print!("{:?} \n\n ",  data_dir.as_path());

//     if !std::path::Path::new(&cache_dir.as_path()).exists() {
//         std::fs::create_dir(&cache_dir.as_path())
//             .expect("failed to create cache directory");
//     }
//     if !std::path::Path::new(&json_dir.as_path()).exists() {
//         File::create(&json_dir.as_path())
//             .expect("failed to create configuration file");
//     }
//     if !std::path::Path::new(&data_dir.as_path()).exists() {
//         File::create(&data_dir.as_path())
//             .expect("failed to create data file");
//     }

//     return (data_dir, json_dir);
// }


/// It takes a `Document` as an argument, and returns a `Result<Client, mongodb::error::Error>`
/// 
/// Arguments:
/// 
/// * `data`: Document - this is the data that we want to send to the server.
/// 
/// Returns:
/// 
/// A Result<Client, mongodb::error::Error>
async fn send_data_to_server(data: Document) -> Result<Client, mongodb::error::Error> {
    let e: mongodb::error::Error;

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://nuc.ie:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("Sensor Relay".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options);

    match client {
        Ok(c) => {
            let data_response = c.database("IOT-Home").collection("Home").insert_one(data, None).await;
            
            match data_response {
                Ok(d) => {
                    println!("Data sent successfully");
                },
                Err(e) => println!("error sending data to server {:?}", e)
            }

            Ok(c)
        },
        Err(e) => return Err(e),
    }

}