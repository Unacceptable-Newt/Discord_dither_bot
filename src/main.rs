mod custom_rejections;
mod discord_structs;
mod dither;

use dotenv::dotenv;
use ed25519_dalek::VerifyingKey;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
//use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::{env, convert::Infallible, str, io::Cursor, sync::mpsc::{self, Sender}};
use warp::{http::{StatusCode, header::HeaderValue}, Filter, reject::{Rejection, reject}, reply::{Reply, self},
hyper::body::Bytes,};
use std::collections::HashMap;
use ed25519::signature::Verifier as BaseVerifier;
use image::{io::Reader, EncodableLayout, ImageFormat};

/// Function for taking a Discord interaction and responding with an apropiate responce.
/// if the ping type is 1 then it is a ping and a pong is replied
/// if the ping type is 2 there is a command interaction and it can be handled by
/// looking at the command recieved. more information can be found in the discord developer
/// documentaion https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-interaction-data-option-structure
async fn return_ping(ping: discord_structs::DiscordPing, post_processor: Sender<(String,String,String,String)>) -> Result<impl warp::Reply, warp::Rejection> {
    if ping.r#type == 1 {
        let mut res = HashMap::new();
        res.insert("type", 1);
        println!("got ping {:?}\n and responding {:?}", ping, res);
        Ok(warp::reply::json(&res))
    }else if ping.r#type == 2{
        let responce = match ping.data {
            None => return Err(warp::reject::custom(custom_rejections::BadBodyError)),
            Some(interaction) => interaction,
        };
        match responce.name.as_str() {
            "test" => {
                let name = match responce.options {
                    None => None,
                    Some(s) => { match s.get(0) {
                        None => None,
                        Some(o) => o.value.to_owned(),
                    }},
                };
                let res = discord_structs::DiscordInteractionResponce::new(name);
                Ok(warp::reply::json(&res))
            }
            "dither" => {
                let resolved = &responce.resolved
                    .ok_or(warp::reject::custom(custom_rejections::BadBodyError))?
                    .attachments.ok_or(warp::reject::custom(custom_rejections::BadBodyError))?;
                let options = &responce.options.ok_or(warp::reject::custom(custom_rejections::BadBodyError))?;
                let flake = &options.first().ok_or(warp::reject::custom(custom_rejections::BadBodyError))?
                    .value;
                let item = match flake {
                    Some(s) => s,
                    None => return Err(warp::reject::custom(custom_rejections::BadBodyError)),
                };
                let url = &resolved.get(item).ok_or(warp::reject::custom(custom_rejections::BadBodyError))?
                    .url;

                let member = match ping.member {
                    Some(m) => m,
                    None => return Err(warp::reject::custom(custom_rejections::BadBodyError)),
                };

                let (username, uid) = match member.user {
                    Some(u) => (u.username, u.id),
                    None => ("BRO".to_string(), "0".to_string()),
                };

                post_processor.send((url.to_string(), ping.token, username, uid)).unwrap();// handle error if it occures

                let res = discord_structs::DiscordInteractionResponce::later_responce();
                Ok(warp::reply::json(&res))
            }
            _ => Err(reject())
        }
    }else {
        Err(reject())
    }
}

/// concatinates the timestamp and body a request into a single string
fn format_header(timestamp: &str, body: &Bytes) -> String {
    //println!("body:\n{:?}",body);
    format!("{}{}",timestamp,str::from_utf8(body).unwrap())
}

/// warp fileter to pass the verifier to the other function in the filter chain
fn with_verifier(verifying_key: VerifyingKey) -> impl Filter<Extract = (VerifyingKey,), Error = Infallible> + Clone {
    warp::any().map(move || verifying_key.clone())
}

struct BodyParser {
    body: Bytes,
    message: String,
}

/// Creates a fileter that gets the timestamp headder and body bytes and
/// makes a verifying message from the two while preserving the bytes in the BodyParser struct
fn get_verifier_message() -> impl Filter<Extract = (BodyParser,), Error = Rejection> + Clone {
    let get_timestamp = warp::header::value("X-Signature-Timestamp");
    let get_bytes = warp::any()//warp::body::content_length_limit(1024* 32)
        .and(warp::body::bytes());
    get_timestamp.and(get_bytes)
        .and_then(|timestamp: HeaderValue, body: Bytes| async move {
            match timestamp.to_str() {
                Ok(s) => {
                    let message = format_header(s, &body);
                    Ok(BodyParser { body, message })
                },
                Err(_) => Err(warp::reject::custom(custom_rejections::InvalidTimestamp)),
            }
        })
}

/// a function that takes the concaternated timestamp and body of the request, 
/// a signature, and a verifying_key and tests if the signature matches the body
/// and timestamp combination. rejecting if the signature is bad and returning the
/// bytes of the body if the signature is good.
async fn verify_header(body_data: BodyParser, signature: HeaderValue, verifying_key: VerifyingKey)
    -> Result<Bytes, Rejection> {
    let signature_string = match signature.to_str() {
        Err(_) => return Err(warp::reject::custom(custom_rejections::BadSignautre)),
        Ok(s) => s,
        };
    //println!("got signature: {}",signature_string);
    let mut signing_key_bytes = [0u8; ed25519_dalek::SIGNATURE_LENGTH];
    if let Err(_) = hex::decode_to_slice(signature_string, &mut signing_key_bytes) {
        return Err(warp::reject::custom(custom_rejections::BadSignautre));
    }
    let signing_key = ed25519_dalek::Signature::from_bytes(&signing_key_bytes);
    match verifying_key.verify(body_data.message.as_bytes(), &signing_key) {
        Ok(()) => {
            Ok(body_data.body)
        },
        Err(_) => {
            Err(warp::reject::custom(custom_rejections::InvalidSignautre))
        },
    }
}

/// a simple function that takes some bytes that represent a json DiscordInteraction object
/// and turns it into the DiscordPing struct. if the json does not have the required fields or
/// is invalid returns a custom rejection.
async fn get_body(body: Bytes) -> Result<discord_structs::DiscordPing, Rejection> {
    match serde_json::from_slice(&body) {
        Ok(ping) => Ok(ping),
        Err(e) => {
            println!("{}",e.to_string());
            Err(warp::reject::custom(custom_rejections::BadBodyError))},
    }
}

#[tokio::main]
async fn main() {
    //loads the values from the .env file
    dotenv().ok();

    let key_string = env::var("PUBLIC_KEY")
        .expect("Discord private key needs to be defined in the .env file");

    let application_id = env::var("APPLICATION_ID")
        .expect("applicaiton id must be specified in the .env file");

    let mut key_bytes = [0u8; ed25519_dalek::PUBLIC_KEY_LENGTH];
    hex::decode_to_slice(key_string, &mut key_bytes).expect("invalid length of ed25519 key");
    let verfying_key = ed25519_dalek::VerifyingKey::from_bytes(&key_bytes).expect("invalid ed25519 key");
    println!("verifying key loaded and correct");

    let get_signature_header = warp::header::value("X-Signature-Ed25519");

    let (tx, rx) = mpsc::channel();

    tokio::spawn(async move {
        let client = reqwest::ClientBuilder::new();
        //let mut headers = HeaderMap::new();
        //let val = format!("Bot {}", auth_token).parse().unwrap();
        //headers.insert(AUTHORIZATION, val);
        let client = client.build().expect("failed to create client");

        loop {
            let (recived, token, username, user_snowflake) = rx.recv().unwrap();
            println!("I have recived an image from {}",username);
            let image_bytes = reqwest::get(recived).await;
            let image_bytes = match image_bytes {
                Ok(v) => v,
                _ => {
                    println!("failed to get image");
                    continue;
                }
            };
            let image_bytes = match image_bytes.bytes().await {
                Ok(v) => v,
                _ => {
                    println!("unable to get image bytes");
                    continue;
                },
            };
                //.bytes().await.unwrap();
            let tmp_reader = Cursor::new(image_bytes.as_bytes());
            let image_reader = match Reader::new(tmp_reader).with_guessed_format() {
                Ok(r) => r,
                _ => {
                    println!("failed to create image reaeder");
                    continue;
                },
            };
            let image_reader = match image_reader.decode() {
                Ok(r) => r,
                _ => {
                    println!("unable to decode image");
                    continue;
                },
            };

            let original_width = image_reader.width();
            let original_height = image_reader.height();
            let mut image_small_reader = image_reader.resize(200, 200, image::imageops::FilterType::Nearest);
            let image_reader = match image_small_reader.as_mut_rgb8() {
                Some(r) => r,
                _ => {
                    println!("failed to get mutiable refference to image");
                    continue;
                },
            };
            dither::dither(image_reader, 4);
            let full_image = image_small_reader.resize(original_width, original_height, image::imageops::FilterType::Nearest);
            let filename = format!("image_{}.png",token);
            let image_path = format!("image/{}",filename);
            let mut image_cursor = Cursor::new(Vec::new());
            let _ = full_image.write_to(&mut image_cursor,ImageFormat::Png);
            let _ = full_image.save(image_path);
            println!("I Have recived recived and processed image");
            let post_url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/@original",application_id,token);
            println!("{}",post_url);
            let responce = client.patch(post_url);

            let form = reqwest::multipart::Form::new();
            let image_part = reqwest::multipart::Part::bytes(image_cursor.into_inner());
            let mut image_type_header = HeaderMap::new();
            image_type_header.insert(CONTENT_TYPE, "image/png".parse().unwrap());
            let image_part = image_part.file_name(filename.clone()).headers(image_type_header);

            let image_object = discord_structs::MessageWithAttachments::new(filename, username, user_snowflake);
            let image_json = serde_json::to_string(&image_object).unwrap();
            let mut json_type_header = HeaderMap::new();
            json_type_header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
            let json_part = reqwest::multipart::Part::text(image_json)
                .headers(json_type_header);

            let form = form
                .part("files[0]", image_part)
                .part("payload_json", json_part);
            
            let reply = match responce.multipart(form).send().await{
                Ok(r) => r,
                Err(e) => {
                    println!("failed to get responce: {e}");
                    continue;
                }
            };
            println!("got a reply with code: {}",reply.status());
        }
    });

    let move_sender = warp::any().map(move || { tx.clone() });

    let verify =  
        warp::any().and(get_verifier_message())
        .and(get_signature_header)
        .and(with_verifier(verfying_key))
        .and_then(verify_header);

    let ping = warp::post()
        .and(warp::path::end())
        .and(verify)
        .and_then(get_body)
        .and(move_sender)
        .and_then(return_ping)
        .recover(handle_rejection);

    warp::serve(ping)
        .run(([127,0,0,1],8080))
        .await;

}

/// handles the various problems that can arise from handling the incomming requests
/// custom errors are located in the custom_rejections file
async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        println!("Failed to Serialize: {}", e.to_string());
        Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
    } else if let Some(_e) = err.find::<custom_rejections::InvalidSignautre>() {
        println!("Bad ed25519 signature");
        Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::UNAUTHORIZED))
    }else {
        eprintln!("Unhandled Rejection: {:?}", err);
        Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
    }
}
