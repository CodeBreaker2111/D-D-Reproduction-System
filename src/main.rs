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
    let mut input = String::from("");

    println!("\n\nHow many generations do you want to run? :");
    input = get_input();
    let generations: i32 = input.trim().parse().unwrap();

    println!("\nRunning {} generations...", input.trim());
    println!("n\n{:?}", combine_parents(parent1, parent2));

    for x in 0..generations {
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

fn combine_subject<T>(set1 : &make_parents::Subject, set2 : &make_parents::Subject) {
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

    subject.MSTR = set1.STR.clone();
    subject.MCON = set1.CON.clone();
    subject.MDEX = set1.DEX.clone();
    subject.MCHR = set1.CHR.clone();
    subject.MWIS = set1.WIS.clone();
    subject.MINT = set1.INT.clone();
    subject.MSEX = set1.SEX.clone();

    subject.DSTR = set2.STR.clone();
    subject.DCON = set2.CON.clone();
    subject.DDEX = set2.DEX.clone();
    subject.DCHR = set2.CHR.clone();
    subject.DWIS = set2.WIS.clone();
    subject.DINT = set2.INT.clone();
    subject.DSEX = set2.SEX.clone();


    subject.STR = combine_chrome(&result_combined_chrome_set1.0, &result_combined_chrome_set2.0);
    subject.CON = combine_chrome(&result_combined_chrome_set1.1, &result_combined_chrome_set2.1);
    subject.DEX = combine_chrome(&result_combined_chrome_set1.2, &result_combined_chrome_set2.2);
    subject.CHR = combine_chrome(&result_combined_chrome_set1.3, &result_combined_chrome_set2.3);
    subject.WIS = combine_chrome(&result_combined_chrome_set1.4, &result_combined_chrome_set2.4);
    subject.INT = combine_chrome(&result_combined_chrome_set1.5, &result_combined_chrome_set2.5);
}

fn combine_chrome(c1 : &String, c2 : &String) -> String {
    let mut return_chrome = String::from("");
    
    if c1.contains("X") {
        if c2.contains("X") {
            return_chrome = String::from("1");
        }
        if !c2.contains("X") {
            let chrome2_int: i32 = c2.parse().unwrap(); // Turn chrome 2 (c2) into int
            let chrome1_int: i32 = c1.replace("X", "").parse().unwrap(); // Turn chrome 1 (c1) into int and strip it of X's
            return_chrome = String::from((chrome1_int * chrome2_int).to_string());
        }
    }

    if c2.contains("X") {
        if c1.contains("X") {
            return_chrome = String::from("1");
        }
        if !c1.contains("X") {
            let chrome1_int: i32 = c1.parse().unwrap(); // Turn chrome 1 (c1) into int
            let chrome2_int: i32 = c2.replace("X", "").parse().unwrap(); // Turn chrome 2 (c2) into int and strip it of X's
            return_chrome = String::from((chrome1_int * chrome2_int).to_string());
        }
    }

    return return_chrome;
}