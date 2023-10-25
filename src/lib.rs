// This crate is a library
#![crate_type = "lib"]

extern crate regex;

use regex::Regex;

use rust_embed::RustEmbed;
use std::{collections::{HashSet, HashMap, VecDeque}, borrow::Cow, io, fs::File};

use clap::{Parser, Subcommand, Args};

use handlebars::Handlebars;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generates a new project from template
    Generate(GenerateArgs),
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    template: Option<String>,
}

#[derive(Args, Debug)]
pub struct ProjectArgs {
    project_name: Option<String>,
}

#[derive(RustEmbed, Debug)]
#[folder = "templates/"]
pub struct Asset;

pub fn render(args:GenerateArgs) {
    let mut folders: HashSet<String> = HashSet::new();
    for tempalte in Asset::iter() {        
        folders.insert(tempalte.split("/").nth(0).unwrap().to_string());
    }

    if let Some(template_name) = args.template {
        if folders.contains(&template_name) {

            println!("Enter project name");
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let p_name: &str = input.trim();
            
            let re = Regex::new(r"\{\{\s*(.*?)\s*\}\}").unwrap();
            let mut data: HashMap<String, String> = HashMap::new();
            let mut t_variables: HashSet<String> = HashSet::new();
            let reg = Handlebars::new();
            
            println!("Generating template {}...", template_name);
            let templates_to_render:Vec<Cow<'static, str>> = Asset::iter()
                .filter(|x| x.starts_with(&template_name))
                .collect();

            println!("Template files:");
            for t in templates_to_render.as_slice(){
                println!("{}", t)
            }
            
            for file in templates_to_render.as_slice(){
                let captured_vars = capture_variables(file, &re);
                let captured_vars_iter = captured_vars.iter();

                for variable in  captured_vars_iter{
                    t_variables.insert(variable.clone());
                }
            }

            populate_data(t_variables, &mut data);

            render_templates(templates_to_render, p_name, reg, &data);

            println!("{:?}", data)

        } else {
            println!("Template for '{}' does not exist", template_name)
        }
    } else {
    
        println!("Avilable templates:");
        for folder in folders {
            println!("{}", folder)
        }
        
    }
}

fn render_templates(templates_to_render: Vec<Cow<'_, str>>, p_name: &str, mut reg: Handlebars<'_>, data: &HashMap<String, String>) {
    for file in templates_to_render.as_slice() {
        let t_content = &Asset::get(&file).unwrap().data;        
        let template_str = std::str::from_utf8(&t_content).unwrap();

        let mut path_q: VecDeque<&str> = file.split("/").collect();
        path_q.pop_front();

        path_q.push_front(&p_name);
         
        let file = std::path::Path::new(path_q.pop_back().unwrap()).file_stem().unwrap();
        path_q.push_back(file.to_str().unwrap());

        let path_v = Vec::from(path_q);
        let path_s = path_v.join("/");

        println!("{}", path_s);

        let path = std::path::Path::new(&path_s);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let fs = File::create(path).unwrap();

        reg.register_template_string("tmp", template_str).unwrap();
        reg.render_to_write("tmp", data, fs).unwrap();
    }
}

fn populate_data(t_variables: HashSet<String>, buf: &mut HashMap<String, String>) {
    println!("Input the correct values for the following template variables: \n");

    for variable in t_variables {
        println!("value for {}: ", variable);
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let result = input.trim();

        buf.insert(variable, result.to_string());
    }
    println!("\n");
}

fn capture_variables(file: &Cow<'_, str>, re: &Regex) -> Vec<String> {
    let t_content = &Asset::get(&file).unwrap().data;
    let s = std::str::from_utf8(&t_content).unwrap();
                
    let values: Vec<String> = re.captures_iter(s)
    .filter_map(|cap| cap.get(1))
    .map(|m| m.as_str().to_owned())
    .collect();

    return values;
}