use oasysdb::prelude::*;
use fastembed::{Embedding};

/*
In short, use Collection to store your vector records or search
similar vectors and use Database to persist a vector collection to the
disk.
*/

/*
let v = Vector((&vector).to_vec());
let m0 = Metadata::Text((&chunk).to_string());
let m1 = Metadata::Float(28.);
let hm = HashMap::from([("key", "value")]);
//let ma = Metadata::Array(vec![m0, m1, hm.into()]);
let record = Record::new(&v, &m0);
*/

/// Takes an embedding (for a chunk) and the chunk text.
pub fn data_to_record(emb: &Embedding, txt: &str) -> Record {
    let vector = Vector((emb).to_vec());
    let metadata = Metadata::Text((&txt).to_string());
    Record::new(&vector, &metadata)
}

pub fn get_db() -> Database {
    //let args = Args::parse(); // Should not be here, have function args instead.
    let db = Database::open("data/test").unwrap();
    // let collection = db.get_collection("vectors").unwrap();
    println!("DB contains {} collections.", db.len());
    db
}

pub fn _save_db(db: &mut Database) {
    let collection = db.get_collection("vectors").unwrap();
    db.save_collection("vectors", &collection).unwrap();
}

pub fn _delete_collection(db: &mut Database, name: &str) {
    let _ = db.delete_collection(name);
}

// We need a save, load, new, ...

/*
    // Replace with your own data.
    //let records = Record::many_random(dimension, 100);

    let config = Config::default();

    // Optionally set the distance function. Default to Euclidean.
    //config.distance = Distance::Cosine;

    // Create a vector collection.
    //let collection = Collection::build(&config, &records).unwrap();
    
    let mut db = Database::open("data/test").unwrap();

    //let collection = Collection::build(&config, &records).unwrap();
    //db.save_collection("vectors", &collection).unwrap();
    let collection = db.get_collection(&args.collection).unwrap();
*/
