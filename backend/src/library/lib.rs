//! This module contains the validation functions
use crate::models::xml_models::Config;
use regex::Regex;

///# Validates the user.
///
///## Arguments
///
/// * `config: Config` - The config struct with the validation rules
/// * `first_name: &str` - The first name of the user
/// * `last_name: &str` - The last name of the user
/// * `age: &str` - The age of the user
/// * `pensum: &str` - The pensum of the user
/// * `location: &str` - The location of the user
/// * `occupation: &str` - The occupation of the user
/// * `ahv_nr: &str` - The ahv number of the user
pub fn validate_user(
    config: Config,
    first_name: &str,
    last_name: &str,
    age: &str,
    pensum: &str,
    location: &str,
    occupation: &str,
    ahv_nr: &str,
) -> Result<(), String> {
    // Extract the configs for the user
    let name_config = &config.validation_rules.person.name;
    let age_config = &config.validation_rules.person.age;
    let pensum_config = &config.validation_rules.person.pensum;
    println!("{:?}", config);
    // check first_name
    if first_name.len() < name_config.min as usize || first_name.len() > name_config.max as usize {
        return Err(format!(
            "Vorname muss mindestens {} Buchstaben lang sein und nicht länger als {} Zeichen.",
            name_config.min, name_config.max
        ));
    }

    // check last_name
    if last_name.len() < name_config.min as usize || last_name.len() > name_config.max as usize {
        return Err(format!(
            "Nachname muss mindestens {} Buchstaben lang sein und nicht länger als {} Zeichen.",
            name_config.min, name_config.max
        ));
    }

    let age_int: u32 = match age.parse::<u32>() {
        Ok(age_int) => age_int,
        Err(_) => return Err("Ungültiges Alter.".to_string()),
    };

    // check age
    if age_int < age_config.min || age_int > age_config.max {
        return Err(format!(
            "Alter muss mindestens {} sein und nicht über {}.",
            age_config.min, age_config.max
        ));
    }

    let pensum_int = match pensum.parse::<u32>() {
        Ok(pensum_int) => pensum_int,
        Err(_) => return Err("Ungültiges Pensum.".to_string()),
    };

    // check pensum
    if pensum_int < pensum_config.min_value || pensum_int > pensum_config.max_value {
        return Err(format!(
            "Pensum muss mindestens {} sein und nicht über {}.",
            pensum_config.min_value, pensum_config.max_value
        ));
    }

    // check location
    if location.len() < 2 || location.len() > 55 {
        return Err(
            "Ort muss mindestens 2 Buchstaben lang sein und nicht länger als 55 Zeichen."
                .to_string(),
        );
    }

    // check occupation
    if occupation.len() < 2 || occupation.len() > 55 {
        return Err(
            "Beruf muss mindestens 2 Buchstaben lang sein und nicht länger als 55 Zeichen."
                .to_string(),
        );
    }

    validate_ahv(ahv_nr)?;

    Ok(())
}

///# Validate optional fields for updates
///
/// Uses the valiadition rules and struct with optional fields to validate the user.
pub fn validate_update(
    config: Config,
    first_name: &Option<String>,
    last_name: &Option<String>,
    age: &Option<String>,
    pensum: &Option<String>,
    location: &Option<String>,
    occupation: &Option<String>,
    ahv_nr: &Option<String>,
) -> Result<(), String> {
    // Extract the configs for the user
    let name_config = config.validation_rules.person.name;
    let age_config = config.validation_rules.person.age;
    let pensum_config = config.validation_rules.person.pensum;

    if let Some(first_name) = first_name {
        if first_name.len() < name_config.min as usize
            || first_name.len() > name_config.max as usize
        {
            return Err(format!(
                "Vorname muss mindestens {} Buchstaben lang sein und nicht länger als {} Zeichen.",
                name_config.min, name_config.max
            ));
        }
    }

    if let Some(last_name) = last_name {
        if last_name.len() < name_config.min as usize || last_name.len() > name_config.max as usize
        {
            return Err(format!(
                "Nachname muss mindestens {} Buchstaben lang sein und nicht länger als {} Zeichen.",
                name_config.min, name_config.max
            ));
        }
    }

    if let Some(age) = age {
        let age_int: u32 = match age.parse::<u32>() {
            Ok(age_int) => age_int,
            Err(_) => return Err("Ungültiges Alter.".to_string()),
        };

        if age_int < age_config.min || age_int > age_config.max {
            return Err(format!(
                "Alter muss mindestens {} sein und nicht über {}.",
                age_config.min, age_config.max
            ));
        }
    }

    if let Some(pensum) = pensum {
        let pensum_int = match pensum.parse::<u32>() {
            Ok(pensum_int) => pensum_int,
            Err(_) => return Err("Ungültiges Pensum.".to_string()),
        };

        if pensum_int < pensum_config.min_value || pensum_int > pensum_config.max_value {
            return Err(format!(
                "Pensum muss mindestens {} sein und nicht über {}.",
                pensum_config.min_value, pensum_config.max_value
            ));
        }
    }

    if let Some(location) = location {
        if location.len() < 2 || location.len() > 55 {
            return Err(
                "Ort muss mindestens 2 Buchstaben lang sein und nicht länger als 55 Zeichen."
                    .to_string(),
            );
        }
    }

    if let Some(occupation) = occupation {
        if occupation.len() < 2 || occupation.len() > 55 {
            return Err(
                "Beruf muss mindestens 2 Buchstaben lang sein und nicht länger als 55 Zeichen."
                    .to_string(),
            );
        }
    }

    if let Some(ahv_nr) = ahv_nr {
        validate_ahv(ahv_nr)?;
    }

    Ok(())
}

///# Validate the ahv number
///
/// This is the algorithm for validating the ahv number.
pub fn validate_ahv(ahv_nr: &str) -> Result<(), String> {
    // check ahv_nr with regex
    let reg_exp = match Regex::new(r"^[7][5][6][.][\d]{4}[.][\d]{4}[.][\d]{2}$") {
        Ok(regex) => regex,
        Err(_) => return Err("Ungültige AHV Nummer.".to_string()),
    };

    if !reg_exp.is_match(&ahv_nr) {
        return Err("Ungültige AHV Nummer.".to_string());
    }

    // Get the last digit
    let check_number = match ahv_nr.chars().last() {
        Some(c) => match c.to_digit(10) {
            Some(digit) => digit as i32,
            None => return Err("Ungültige AHV Nummer.".to_string()),
        },
        None => return Err("Ungültige AHV Nummer.".to_string()),
    };

    // Remove the last digit
    let ahv_nr = &ahv_nr[0..ahv_nr.len() - 1];

    // Remove the dots and reverse the string
    let normalized = ahv_nr.replace(".", "").chars().rev().collect::<String>();

    // calculate for each digit the sum by multiplying every second digit by 3
    let mut sum = 0;
    for (i, digit) in normalized.chars().enumerate() {
        let digit = match digit.to_digit(10) {
            Some(digit) => digit as i32,
            None => return Err("Invalid digit".into()), // or handle the error as you see fit
        };

        let add = if i % 2 == 0 { digit * 3 } else { digit };
        sum += add;
    }

    // Calculate the next ten by rounding up the sum divided by 10 and multiply by 10
    let next_ten = ((sum as f32 / 10.0).ceil() * 10.0) as i32;

    // Calculate the check sum by subtracting the sum from the next ten
    let check_sum = next_ten - sum;
    if check_sum != check_number {
        return Err("Ungültige AHV Nummer.".to_string());
    }
    Ok(())
}

// TESTS
