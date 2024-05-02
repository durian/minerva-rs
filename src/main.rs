use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use oasysdb::prelude::*;
use clap::Parser;
use std::collections::HashMap;
mod database;
use database::{get_db};
mod embedder;
use embedder::{chunk_string, embed_file_txt};


// =====================================================================
// Command line arguments.
// =====================================================================

#[derive(Parser, Debug, Clone)]
struct Args {
    // Filename
    #[arg(short, long, help = "The file... but what is it?")]
    pub filename: Option<String>, // Path thingy?

    // Chunk size
    #[clap(long, action, default_value_t = 250, help = "Chunk size in characters.")]
    pub chunksize: usize,

    // Name of the database (collection)
    #[arg(long, default_value = "vectors", help = "Name of the database collection.")]
    pub collection: String,

    // Query
    #[arg(short, long, help = "Question?")]
    pub query: Option<String>,

    // Extra output
    #[arg(long, short, action, help = "Produce superfluous output.")]
    pub verbose: bool,
}

// =====================================================================
// Main.
// =====================================================================

fn main() -> anyhow::Result<()> {

    let args = Args::parse();
    dbg!("{:?}", &args);
    //let filename = &args.filename;

    //println!("{:?}", embed_file_txt("texts/water.txt", args.chunksize));
    
    // With default InitOptions
    //let model = TextEmbedding::try_new(Default::default()).expect("Cannot initialise model.");
    //dbg!(TextEmbedding::list_supported_models());
    
    // With custom InitOptions
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_progress: true,
        ..Default::default()
    }).expect("Cannot Initialise model.");

    let documents = vec![
        "passage: Hello, World!",
        "query: Hello, World!",
        "passage: This is an example passage.",
        // You can leave out the prefix but it's recommended
        "fastembed-rs is licensed under Apache  2.0"
    ];
    
    // Generate embeddings with the default batch size, 256
    let embeddings = model.embed(documents, None).expect("Cannot create embeddings.");
    //println!("{:?}", embeddings);
    
    println!("Embeddings count: {}", embeddings.len()); // -> Embeddings length: 4
    println!("Embedding dimension: {}", embeddings[0].len()); // -> Embedding dimension: 384

    // ----

    // Optionally set the distance function. Default to Euclidean.
    //config.distance = Distance::Cosine;

    // Create a vector collection.
    //let collection = Collection::build(&config, &records).unwrap();

    let data = embed_file_txt("texts/water.txt", args.chunksize).unwrap();
    let vectors = model.embed(data.clone(), None).expect("Cannot create embeddings.");
    let mut records = vec![];
    for (chunk, vector) in data.iter().zip(vectors.iter()) {
        let v = Vector((&vector).to_vec());
        let m0 = Metadata::Text((&chunk).to_string());
        let m1 = Metadata::Float(28.);
        let hm = HashMap::from([("key", "value")]);
        //let ma = Metadata::Array(vec![m0, m1, hm.into()]);
        let record = Record::new(&v, &m0);
        println!("Record {:?}", m0);
        records.push(record);
    }
    
    // This is the saved DB, containing different collections.
    let mut db = get_db();
    let mut collection = db.get_collection(&args.collection).unwrap_or_else(|_| {
        println!("Creating a new empty collection.");
        let config = Config::default();
        //Collection::build(&config, &records).unwrap()
        let c = Collection::new(&config);
        db.save_collection(&args.collection, &c).unwrap(); // Save it so it exists on disk.
        /*
        match db.save_collection(&args.collection, &c) {
            Ok(_) => c,
            Err(e) => {
                eprintln!("Failed to save the new collection: {}", e);
                panic!("Critical error: could not save collection");
            }
        }
        */
        c
    });
    
    //let ids = collection.insert_many(&records).unwrap();
    //println!("{:?}", ids);
    //db.save_collection(&args.collection, &collection).unwrap(); // Save it so it exists on disk.
    println!("DB now contains {} items.", db.len());

    let list = collection.list().unwrap();
    //println!("{:?}", list);
    
    // Search for the nearest neighbours.
    if let Some(query) = &args.query {
        println!("Asking {}", &query);
        
        let data = chunk_string(query, args.chunksize);
        println!("{:?}", data);
        let vectors = model.embed(data, None).expect("Cannot create embeddings.");
        let v = vectors.get(0).expect("uh");
        let query = Vector((&v).to_vec());
        let result = collection.search(&query, 8).unwrap();
        
        for res in result {
            //println!("{:?}", res);
            let md = match res.data {
                Metadata::Text(value) => value,
                _ => "Data is not a text.".to_string()
            };
            let (id, distance) = (res.id, res.distance);
            println!("{distance:.5} | ID: {id} {md}");
        }
    }

    Ok(())
}
