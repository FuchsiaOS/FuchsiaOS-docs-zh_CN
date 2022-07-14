use std::{env, process};
use std::fs::File;
use std::io::{Read, Write};


use regex::Regex;
use serde::Deserialize;

/*
 *       Author  :   Ray
 *       email   :   wybuhui@sina.com
 *       Data    :   2022-07-14
 *       Function:   take a note for translation
 *       Requirements:   none
 *       Usage   :   trans_note [confile file] [input file] [out file]
 *       Platforms:  All
 **/


#[derive(Deserialize)]
struct Config {
    item: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    author: String,
    pattern: String,
    sub_str: String,
}

fn help(program : &str) {
    println!("Usage: {} [-hioc] [options]", program);
    println!("\toptions:");
    println!("\t\t -h : help information");
    println!("\t\t -c : config file, followed by config file name");
    println!("\t\t -i : input file, followed by input file name");
    println!("\t\t -o : reuslt file, followed by result file name");
    println!("\n\t Default options : -c config.toml -i input.txt -o result.txt");
    println!("\n\n\tAuthor: Ray");
    process::exit(0);
}

fn parse_args(args: &[String]) -> Result<(&str, &str, &str), &str>{

    let mut con_file: &str = "config.toml";
    let mut in_file: &str = "input.txt";
    let mut out_file: &str = "result.txt";

    let args_len = args.len();
    let program: &String = args
        .get(0)
        .clone()
        .unwrap();


    for mut i in 1..args_len {
        let tmp = args
            .get(i)
            .clone()
            .unwrap();

        match tmp.as_str() {
            "-h" => help(program),
            "-c" => {
                i += 1;
                con_file = args
                    .get(i)
                    .clone()
                    .unwrap();
            },
            "-i" => {
                i += 1;
                in_file = args
                    .get(i)
                    .clone()
                    .unwrap();
            },
            "-o" => {
                i += 1;
                out_file = args
                    .get(i)
                    .clone()
                    .unwrap();
            },
            _ => (),
        }
    }

    Ok((con_file, in_file, out_file))
}

fn main() {
    let args: Vec<String>  = env::args().collect();

    let con_file: &str;
    let in_file: &str;
    let out_file: &str;


    match parse_args(&args) {
        Ok((con_file_a, in_file_a, out_file_a))  => {
            con_file = con_file_a;
            in_file = in_file_a;
            out_file = out_file_a;
        },
        Err(_e) => process::exit(1),
    }

    println!("con : {}, in : {}, out : {}", con_file, in_file, out_file);




    let mut con_f = File::open(con_file)
        .unwrap_or_else(|err| {
            println!("open {} failed : {}", con_file, err);
            process::exit(1);
        });
    let mut in_f = File::open(in_file)
        .unwrap_or_else(|err| {
            println!("open {} failed : {}", in_file, err);
            process::exit(1);
        });
    let mut out_f = File::create(out_file)
        .unwrap_or_else(|err| {
            println!("create {} failed : {}", out_file, err);
            process::exit(1);
        });

    let mut content = String::new();




    con_f.read_to_string(&mut content).unwrap();
    let all_item: Config = toml::from_str(&content).unwrap();

    content.clear();
    in_f.read_to_string(&mut content).unwrap();



    println!("start to working ...");

    for item in all_item.item.iter() {
        println!("author {}, pattern \"{}\" ...", item.author, item.pattern);


        let re = Regex::new(item.pattern.as_str()).unwrap();
        content = re 
            .replace_all(content.as_str(), item.sub_str.as_str())
            .to_string();

    }

    
    out_f.write_all(content.as_bytes())
        .expect("write result content failed!");

}
