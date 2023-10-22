mod make_parents;
mod combination;

use rand::Rng;

pub fn main() {
    let parents = make_parents::main();
    println!("Subject 1 :\n\n{:?}\n\nSubject 2 :\n\n{:?}", parents.0, parents.1);
    //combination::main(parents.0, parents.1);
    combination_main(parents.0, parents.1);
}

use std::io;

fn combination_main(parent1 : make_parents::Subject, parent2: make_parents::Subject) {
    let dummy_subject = make_parents::Subject {
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

    let mut input = String::from("");

    println!("\n\nHow many generations do you want to run? :");
    input = get_input();
    let generations: i32 = input.trim().parse().unwrap();

    println!("\nRunning {} generations...", input.trim());
    println!("n\n{:?}", combine_parents(parent1.clone(), parent2.clone()));

    let mut last_gen_subjects = (dummy_subject.clone(), dummy_subject.clone());

    for x in 0..generations {
        if x == 0 {
            println!("Hello World!");
            last_gen_subjects.0 = combine_parents(parent1.clone(), parent2.clone()); println!("\n\n{:?}", last_gen_subjects.0);
            last_gen_subjects.1 = combine_parents(parent1.clone(), parent2.clone()); println!("\n\n{:?}", last_gen_subjects.1);

            last_gen_subjects.0 = combine_subject(&last_gen_subjects.0, &last_gen_subjects.1); println!("\n\n{:?}", last_gen_subjects.0);
            last_gen_subjects.1 = combine_subject(&last_gen_subjects.0, &last_gen_subjects.1); println!("\n\n{:?}", last_gen_subjects.1);
        }

        println!("Generation {} complete.", (x + 1).to_string());
    }
}

fn get_input() -> String {
    let mut input = String::from("");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn combine_parents(mom : make_parents::Subject, dad: make_parents::Subject) -> make_parents::Subject {
    let mut subject = make_parents::Subject {
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

    (subject.MSTR, subject.MCON, subject.MDEX, subject.MCHR, subject.MWIS, subject.MINT, subject.MSEX,) = (mom.STR, mom.CON, mom.DEX, mom.CHR, mom.WIS, mom.INT, mom.SEX,);
    (subject.DSTR, subject.DCON, subject.DDEX, subject.DCHR, subject.DWIS, subject.DINT, subject.DSEX,) = (dad.STR, dad.CON, dad.DEX, dad.CHR, dad.WIS, dad.INT, dad.SEX,);
    
    return subject;
}

fn choose_chrome(c1 : String, c2 : String) -> String {
    let mut return_value = String::from("");
    let chrome_value = rand::thread_rng().gen_range(1..=2);

    if chrome_value == 1 {
        return_value = c1;
    }

    if chrome_value == 2 {
        return_value = c2;
    }
    
    return return_value;
}

fn combine_chrome_set(set : make_parents::Subject) -> (String, String, String, String, String, String, String) {
    let mut return_strings = (String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""));

    return_strings = (choose_chrome(set.MSTR, set.DSTR), choose_chrome(set.MCON, set.DCON), choose_chrome(set.MDEX, set.DDEX), choose_chrome(set.MCHR, set.DCHR), choose_chrome(set.MWIS, set.DWIS), choose_chrome(set.MINT, set.DINT), choose_chrome(set.MSEX, set.DSEX));
    
    return return_strings;
}

fn combine_subject(set1 : &make_parents::Subject, set2 : &make_parents::Subject) -> make_parents::Subject {
    let non_ref_set1: make_parents::Subject = set1.clone();
    let non_ref_set2: make_parents::Subject = set2.clone();
    let result_combined_chrome_set1 = combine_chrome_set(non_ref_set1);
    let result_combined_chrome_set2 = combine_chrome_set(non_ref_set2);

    let mut subject = make_parents::Subject {
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

    if rand::thread_rng().gen_range(1..=2) == 1 {subject.MSTR = set1.MSTR.clone();} else {subject.MSTR = set1.DSTR.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.MCON = set1.MCON.clone();} else {subject.MCON = set1.DCON.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.MDEX = set1.MDEX.clone();} else {subject.MDEX = set1.DDEX.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.MCHR = set1.MCHR.clone();} else {subject.MCHR = set1.DCHR.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.MWIS = set1.MWIS.clone();} else {subject.MWIS = set1.DWIS.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.MINT = set1.MINT.clone();} else {subject.MINT = set1.DINT.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.MSEX = set1.MSEX.clone();} else {subject.MSEX = set1.DSEX.clone();}

    if rand::thread_rng().gen_range(1..=2) == 1 {subject.DSTR = set1.MSTR.clone();} else {subject.DSTR = set1.DSTR.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.DCON = set1.MCON.clone();} else {subject.DCON = set1.DCON.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.DDEX = set1.MDEX.clone();} else {subject.DDEX = set1.DDEX.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.DCHR = set1.MCHR.clone();} else {subject.DCHR = set1.DCHR.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.DWIS = set1.MWIS.clone();} else {subject.DWIS = set1.DWIS.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.DINT = set1.MINT.clone();} else {subject.DINT = set1.DINT.clone();}
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.DSEX = set1.MSEX.clone();} else {subject.DSEX = set1.DSEX.clone();}


    subject.STR = combine_chrome(&result_combined_chrome_set1.0, &result_combined_chrome_set2.0);
    subject.CON = combine_chrome(&result_combined_chrome_set1.1, &result_combined_chrome_set2.1);
    subject.DEX = combine_chrome(&result_combined_chrome_set1.2, &result_combined_chrome_set2.2);
    subject.CHR = combine_chrome(&result_combined_chrome_set1.3, &result_combined_chrome_set2.3);
    subject.WIS = combine_chrome(&result_combined_chrome_set1.4, &result_combined_chrome_set2.4);
    subject.INT = combine_chrome(&result_combined_chrome_set1.5, &result_combined_chrome_set2.5);
    if rand::thread_rng().gen_range(1..=2) == 1 {subject.SEX = String::from("X")} else {subject.SEX = String::from("Y")}

    println!("{:?}", subject);

    return subject;
}

fn combine_chrome(c1 : &String, c2 : &String) -> String {
    let mut return_chrome = String::from("");
    
    if c1.contains("X") {
        if c2.contains("X") {
            return_chrome = String::from("1");
        }
        if !c2.contains("X") {
            let chrome2_int: i32 = c2.parse().unwrap(); // Turn chrome 2 (c2) into int
            let chrome1_int: f32 = c1.replace("X", "").parse().unwrap(); // Turn chrome 1 (c1) into int and strip it of X's
            return_chrome = String::from((chrome1_int as i32 * chrome2_int).to_string());
        }
    }

    if c2.contains("X") {
        if c1.contains("X") {
            return_chrome = String::from("1");
        }
        if !c1.contains("X") {
            let chrome1_int: i32 = c1.parse().unwrap(); // Turn chrome 1 (c1) into int
            let chrome2_int: f32 = c2.replace("X", "").parse().unwrap(); // Turn chrome 2 (c2) into int and strip it of X's
            return_chrome = String::from((chrome1_int * chrome2_int as i32).to_string());
        }
    }

    println!("{}", return_chrome);

    return return_chrome;
}