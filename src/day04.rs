// todo: test all individual closure conditions in
// `is_valid`
use crate::helpers::read;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Passport {
    pub byr: Option<String>, // birth year
    pub iyr: Option<String>, // issue year
    pub eyr: Option<String>, // expiration year
    pub cid: Option<String>, // country id
    pub pid: Option<String>, // passport id
    pub hgt: Option<String>, // height
    pub hcl: Option<String>, // hair colour
    pub ecl: Option<String>, // eye colour
}

impl Passport {
    /// Panics if key cannot be matched.
    pub fn add_key(&mut self, key: &str, value: &str) {
        let get = || -> Option<String> { Some(value.to_owned()) };

        match key {
            "byr" => self.byr = get(),
            "iyr" => self.iyr = get(),
            "eyr" => self.eyr = get(),
            "cid" => self.cid = get(),
            "pid" => self.pid = get(),
            "hgt" => self.hgt = get(),
            "hcl" => self.hcl = get(),
            "ecl" => self.ecl = get(),
            _ => panic!("Unexpected field key for Passport."),
        }
    }

    pub fn has_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.pid.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
    }

    pub fn is_valid(&self) -> bool {
        // closure to validate range of numerical values
        let check_range = |value: Option<u16>, min: u16, max: u16| -> bool {
            if let Some(x) = value {
                if min <= x && x <= max {
                    return true;
                }
            }
            false
        };

        // closure to validate year values
        let check_year_range = |value: &Option<String>, min: u16, max: u16| -> bool {
            if let Some(str) = value {
                if str.chars().count() != 4 {
                    return false;
                }

                let year: Option<u16> = str.parse().ok();
                if year.is_none() {
                    return false;
                }

                return check_range(year, min, max);
            }
            false
        };

        let check_pid = || -> bool {
            if let Some(str) = &self.pid {
                if str.chars().count() == 9 && str.chars().all(|c| c.is_numeric()) {
                    return true;
                }
            }
            false
        };

        let check_hgt = || -> bool {
            if let Some(str) = &self.hgt {
                if str.len() < 2 {
                    return false;
                }

                let unit = &str[str.len() - 2..];
                let size = &str[0..str.len() - 2];

                // tests if unit of measurement is correct
                if unit != "cm" && unit != "in" {
                    return false;
                }

                // tests if there is a valid numerical value
                let size: Option<u16> = size.parse().ok();
                if size.is_none() {
                    return false;
                }

                // tests if values are in the correct range of the units
                if unit == "cm" {
                    return check_range(size, 150, 193);
                }

                if unit == "in" {
                    return check_range(size, 59, 76);
                }
            }
            false
        };

        let check_hcl = || -> bool {
            if let Some(str) = &self.hcl {
                // check for correct size
                if str.chars().count() != 7 {
                    return false;
                }

                // check characters
                let mut chars = str.chars();
                let ch0 = chars.next();
                if ch0 != Some('#') {
                    return false;
                }

                if chars.all(|c| c.is_numeric() || ('a' <= c && c <= 'f')) {
                    return true;
                }
            }
            false
        };

        let check_ecl = || -> bool {
            if let Some(str) = &self.ecl {
                match &str[..] {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
                    _ => return false,
                }
            }
            false
        };

        let byr = check_year_range(&self.byr, 1920, 2002);
        let iyr = check_year_range(&self.iyr, 2010, 2020);
        let eyr = check_year_range(&self.eyr, 2020, 2030);
        let pid = check_pid();
        let hgt = check_hgt();
        let hcl = check_hcl();
        let ecl = check_ecl();

        byr && iyr && eyr && pid && hgt && hcl && ecl
    }
}

// passport data is separated by spaces or new lines
// passport entries are separated by blank lines
/// # Assumptions
///
/// - input data is well behaved.
pub fn parse_input(input: &str) -> Vec<Passport> {
    let mut ret = vec![Passport::default()];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            // add next entry if blank line
            ret.push(Passport::default());
            continue;
        }
        let split_at: &[_] = &[' ', ':'];
        let mut split = line.split(split_at).peekable();

        while split.peek().is_some() {
            let key = split.next().unwrap();
            let value = split.next().expect("Key without value.");
            ret.last_mut()
                .expect("Output Vec `ret` is empty.")
                .add_key(key, value);
        }
    }
    ret
}

pub fn count_valid_simple(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.has_required_fields()).count()
}

pub fn count_valid_complete(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    let str = read::read_to_str("day04").unwrap();
    let passports = parse_input(&str);
    println!("Day 04");
    println!(
        "Total of valid passports by simple method: {}",
        count_valid_simple(&passports)
    );
    println!(
        "Total of valid passports by complete method: {}",
        count_valid_complete(&passports)
    );
    println!();
}

//--------------------------------------------------------------------
// Testes
//--------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    const INPUT_STR0: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INPUT_STR_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const INPUT_STR_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    lazy_static! {
        static ref INPUT: Vec<Passport> = parse_input(INPUT_STR0);
        static ref INPUT_INVALID: Vec<Passport> = parse_input(INPUT_STR_INVALID);
        static ref INPUT_VALID: Vec<Passport> = parse_input(INPUT_STR_VALID);
    }

    #[test]
    fn test_parse_input() {
        let parsed = vec![
            Passport {
                ecl: Some("gry".to_owned()),
                pid: Some("860033327".to_owned()),
                eyr: Some("2020".to_owned()),
                hcl: Some("#fffffd".to_owned()),
                byr: Some("1937".to_owned()),
                iyr: Some("2017".to_owned()),
                cid: Some("147".to_owned()),
                hgt: Some("183cm".to_owned()),
            },
            Passport {
                iyr: Some("2013".to_owned()),
                ecl: Some("amb".to_owned()),
                cid: Some("350".to_owned()),
                eyr: Some("2023".to_owned()),
                pid: Some("028048884".to_owned()),
                hcl: Some("#cfa07d".to_owned()),
                byr: Some("1929".to_owned()),
                ..Passport::default()
            },
            Passport {
                hcl: Some("#ae17e1".to_owned()),
                iyr: Some("2013".to_owned()),
                eyr: Some("2024".to_owned()),
                ecl: Some("brn".to_owned()),
                pid: Some("760753108".to_owned()),
                byr: Some("1931".to_owned()),
                hgt: Some("179cm".to_owned()),
                ..Passport::default()
            },
            Passport {
                hcl: Some("#cfa07d".to_owned()),
                eyr: Some("2025".to_owned()),
                pid: Some("166559648".to_owned()),
                iyr: Some("2011".to_owned()),
                ecl: Some("brn".to_owned()),
                hgt: Some("59in".to_owned()),
                ..Passport::default()
            },
        ];
        assert_eq!(parsed, *INPUT);
    }

    #[test]
    fn test_add_key() {
        let mut p = Passport::default();
        p.add_key("ecl", "gry");
        p.add_key("pid", "860033327");
        p.add_key("eyr", "2020");
        p.add_key("hcl", "#fffffd");
        p.add_key("byr", "1937");
        p.add_key("iyr", "2017");
        p.add_key("cid", "147");
        p.add_key("hgt", "183cm");
        assert_eq!(p, INPUT[0]);
    }

    #[test]
    fn test_has_required_fields() {
        // valid
        assert!(INPUT[0].has_required_fields()); // all fields
        assert!(INPUT[2].has_required_fields()); // only cid missing

        // invalid
        assert!(!INPUT[1].has_required_fields()); // missing hgt
        assert!(!INPUT[3].has_required_fields()); // missing cid and byr
    }

    #[test]
    fn test_count_valid() {
        assert_eq!(count_valid_simple(&INPUT), 2);
    }

    #[test]
    fn test_is_valid() {
        assert!(INPUT_INVALID.iter().all(|p| !p.is_valid()));
        assert!(INPUT_VALID.iter().all(|p| p.is_valid()));
    }
}
