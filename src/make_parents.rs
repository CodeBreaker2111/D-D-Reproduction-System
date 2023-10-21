use std::io;
use easy_file::*;
use rand::Rng;

#[derive(Debug)]
#[derive(Clone)]
pub struct Subject {
    pub STR : String,
    pub CON : String,
    pub DEX : String,
    pub CHR : String,
    pub WIS : String,
    pub INT : String,
    pub SEX : String,
    pub MSTR : String,
    pub MCON : String,
    pub MDEX : String,
    pub MCHR : String,
    pub MWIS : String,
    pub MINT : String,
    pub MSEX : String,
    pub DSTR : String,
    pub DCON : String,
    pub DDEX : String,
    pub DCHR : String,
    pub DWIS : String,
    pub DINT : String,
    pub DSEX : String
}

pub fn main() -> (Subject, Subject) {
    let mut subject1 = Subject {
        STR : String::from(""),
        CON : String::from(""),
        DEX : String::from(""),
        CHR : String::from(""),
        WIS : String::from(""),
        INT : String::from(""),
        SEX : String::from(""),
        MSTR : String::from(""),
        MCON : String::from(""),
        MDEX : String::from(""),
        MCHR : String::from(""),
        MWIS : String::from(""),
        MINT : String::from(""),
        MSEX : String::from(""),
        DSTR : String::from(""),
        DCON : String::from(""),
        DDEX : String::from(""),
        DCHR : String::from(""),
        DWIS : String::from(""),
        DINT : String::from(""),
        DSEX : String::from("")

    };

    let mut subject2 = Subject {
        STR : String::from(""),
        CON : String::from(""),
        DEX : String::from(""),
        CHR : String::from(""),
        WIS : String::from(""),
        INT : String::from(""),
        SEX : String::from(""),
        MSTR : String::from(""),
        MCON : String::from(""),
        MDEX : String::from(""),
        MCHR : String::from(""),
        MWIS : String::from(""),
        MINT : String::from(""),
        MSEX : String::from(""),
        DSTR : String::from(""),
        DCON : String::from(""),
        DDEX : String::from(""),
        DCHR : String::from(""),
        DWIS : String::from(""),
        DINT : String::from(""),
        DSEX : String::from("")

    };

    println!("Please enter name of subject :");

    let mut input = get_input();

    if let Ok(_r) = create_dir!("./subjects") {} ;
    match  create_write_file!(std::path::Path::new("./subjects/subject1.txt"),"Subject Name : ".as_bytes()){
        Ok(_r)=>{},
        Err(_e)=>{}
    }

    match append_to_file!("./subjects/subject1.txt", format!("{}\n", input).as_bytes()) {
        Ok(_r) => {}
        Err(_e) => {}
    }

    print!("\n");

    let mut STR = String::from("STR"); let STR_chrome1 =  generate_chrome(&STR); subject1.STR = STR_chrome1.1; let STR_chrome2 =  generate_chrome(&STR); subject2.STR = STR_chrome2.1;
    let mut CON = String::from("CON"); let CON_chrome1 =  generate_chrome(&CON); subject1.CON = CON_chrome1.1; let CON_chrome2 =  generate_chrome(&CON); subject2.CON = CON_chrome2.1;
    let mut DEX = String::from("DEX"); let DEX_chrome1 =  generate_chrome(&DEX); subject1.DEX = DEX_chrome1.1; let DEX_chrome2 =  generate_chrome(&DEX); subject2.DEX = DEX_chrome2.1;
    let mut CHR = String::from("CHR"); let CHR_chrome1 =  generate_chrome(&CHR); subject1.CHR = CHR_chrome1.1; let CHR_chrome2 =  generate_chrome(&CHR); subject2.CHR = CHR_chrome2.1;
    let mut WIS = String::from("WIS"); let WIS_chrome1 =  generate_chrome(&WIS); subject1.WIS = WIS_chrome1.1; let WIS_chrome2 =  generate_chrome(&WIS); subject2.WIS = WIS_chrome2.1;
    let mut INT = String::from("INT"); let INT_chrome1 =  generate_chrome(&INT); subject1.INT = INT_chrome1.1; let INT_chrome2 =  generate_chrome(&INT); subject2.INT = INT_chrome2.1;
    let SEX_chrome1 = generate_sex(); subject1.SEX = SEX_chrome1.1;


    match append_to_file!("./subjects/subject1.txt", format!("{}\n", STR_chrome1.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject1.txt", format!("{}\n", CON_chrome1.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject1.txt", format!("{}\n", DEX_chrome1.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject1.txt", format!("{}\n", CHR_chrome1.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject1.txt", format!("{}\n", WIS_chrome1.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject1.txt", format!("{}\n", INT_chrome1.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject1.txt", format!("{}\n", SEX_chrome1.0).as_bytes()) {Ok(_r) => {println!("\ndone");}Err(_e) => {}}

    println!("Please enter name of subject :");

    input = get_input();

    match  create_write_file!(std::path::Path::new("./subjects/subject2.txt"),"Subject Name : ".as_bytes()){
        Ok(_r)=>{},
        Err(_e)=>{}
    }

    match append_to_file!("./subjects/subject2.txt", format!("{}\n", input).as_bytes()) {
        Ok(_r) => {}
        Err(_e) => {}
    }

    print!("\n");

    let SEX_chrome2 = generate_sex(); subject2.SEX = SEX_chrome2.1;

    match append_to_file!("./subjects/subject2.txt", format!("{}\n", STR_chrome2.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject2.txt", format!("{}\n", CON_chrome2.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject2.txt", format!("{}\n", DEX_chrome2.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject2.txt", format!("{}\n", CHR_chrome2.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject2.txt", format!("{}\n", WIS_chrome2.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject2.txt", format!("{}\n", INT_chrome2.0).as_bytes()) {Ok(_r) => {}Err(_e) => {}}
    match append_to_file!("./subjects/subject2.txt", format!("{}\n", SEX_chrome2.0).as_bytes()) {Ok(_r) => {println!("\ndone");}Err(_e) => {}}

    return (subject1, subject2);

}

fn get_input() -> String {
    let mut input = String::from("");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn generate_chrome(chrome: &String) -> (String, String) {
    let mut return_value = (String::from(""), String::from(""));
    let m_chance = rand::thread_rng().gen_range(1..=4);

    if m_chance == 1 {
        let multiplier = rand::thread_rng().gen_range(5..=30);
        let float_multiplier = multiplier as f64 / 10.0;

        let string_multiplier = float_multiplier.to_string();

        return_value.0 = format!("{} : X{}", chrome, string_multiplier);
        return_value.1 = format!("X{}", string_multiplier);
    }

    if m_chance != 1 {
        let value = rand::thread_rng().gen_range(1..=18);
        let string_value = value.to_string();

        return_value.0 = format!("{} : {}", chrome, string_value);
        return_value.1 = format!("{}", string_value);
    }

    return return_value;
}

fn generate_sex() -> (String, String) {
    let mut sex_string = (String::from(""), String::from(""));

    println!("Do you want to decide the gender of the subject or let the program decide (L: let program decide, D: decide myself) ? ");
    let decision = get_input();

    if decision.trim() == "D" {
        println!("What gender is the subject (X: Female, Y: Male) ?");
        let decision2 = get_input();

        if decision2.trim() == "X" {
            sex_string.0 = format!("SEX : {}", "X");
            sex_string.1 = String::from("X");
        }

        if decision2.trim() == "Y" {
            sex_string.0 = format!("SEX : {}", "Y");
            sex_string.1 = String::from("Y");
        }
    }

    if decision.trim() == "L" {
        let sex = rand::thread_rng().gen_range(1..=2);

        if sex == 1 {
            sex_string.0 = format!("SEX : {}", "X");
            sex_string.1 = String::from("X");
        }

        if sex == 2 {
            sex_string.0 = format!("SEX : {}", "Y");
            sex_string.1 = String::from("Y");
        }
    }

    return sex_string;
}