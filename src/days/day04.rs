use crate::day::Day;
use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;

#[derive(Debug)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let mut ret = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        let key_values = line.trim().split(|c| c == ' ' || c == '\n').map(|x| {
            let xsplit: Vec<&str> = x.trim().split(':').collect();
            (xsplit[0], xsplit[1])
        });
        for (key, value) in key_values {
            match key {
                "byr" => ret.byr = Some(value.to_string()),
                "iyr" => ret.iyr = Some(value.to_string()),
                "eyr" => ret.eyr = Some(value.to_string()),
                "hgt" => ret.hgt = Some(value.to_string()),
                "hcl" => ret.hcl = Some(value.to_string()),
                "ecl" => ret.ecl = Some(value.to_string()),
                "pid" => ret.pid = Some(value.to_string()),
                "cid" => ret.cid = Some(value.to_string()),
                _ => (),
            }
        }
        Ok(ret)
    }
}

pub struct Day04 {}

impl Day04 {
    fn check_valid_2(&self, p: &Passport) -> Result<bool, String> {
        let byr = p
            .byr
            .as_ref()
            .ok_or("Missing Birth Year")?
            .parse::<isize>()
            .map_err(|e| e.to_string())?;
        if byr > 2002 || byr < 1920 {
            return Err("Invalid Birth Year".to_string());
        }
        let iyr = p
            .iyr
            .as_ref()
            .ok_or("Missing Issue Year")?
            .parse::<isize>()
            .map_err(|e| e.to_string())?;
        if iyr > 2020 || iyr < 2010 {
            return Err("Invalid Issue Year".to_string());
        }
        let eyr = p
            .eyr
            .as_ref()
            .ok_or("Missing Expiry Year")?
            .parse::<isize>()
            .map_err(|e| e.to_string())?;
        if eyr > 2030 || eyr < 2020 {
            return Err("Invalid Expiry Year".to_string());
        }
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^([0-9]{2,3})(in|cm)$").unwrap();
        }
        let hgt = HGT_RE
            .captures(p.hgt.as_ref().ok_or("Missing Height")?)
            .ok_or("Invalid Height")?;
        if match &hgt[2] {
            "cm" => {
                let h = hgt[1].parse::<isize>().map_err(|e| e.to_string())?;
                h < 150 || h > 193
            }
            "in" => {
                let h = hgt[1].parse::<isize>().map_err(|e| e.to_string())?;
                h < 59 || h > 76
            }
            _ => true,
        } {
            return Err("Invalid Height".to_string());
        }
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^.[0-9a-f]{6}$").unwrap();
        }
        if !HCL_RE.is_match(p.hcl.as_ref().ok_or("Missing Hair Color")?) {
            return Err("Invalid Hair Color".to_string());
        }
        if !match p.ecl.as_ref().ok_or("Missing Eye Color")?.as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        } {
            return Err("Invalid Eye Color".to_string());
        }
        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        if !PID_RE.is_match(p.pid.as_ref().ok_or("Missing Passport ID")?) {
            return Err("Invalid Passport ID".to_string());
        }

        Ok(true)
    }
}

impl Day for Day04 {
    type InputElement = Passport;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        input
            .into_iter()
            .filter(|p| {
                p.byr != None
                    && p.iyr != None
                    && p.eyr != None
                    && p.hgt != None
                    && p.hcl != None
                    && p.ecl != None
                    && p.pid != None
            })
            .count()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        input
            .into_iter()
            .filter(|p| {
                let ret = self.check_valid_2(*p);
                ret.is_ok() && ret.unwrap()
            })
            .count()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .split("\n\n")
            .map(|x| x.parse::<Self::InputElement>().expect("Invalid input"))
            .collect()
    }
}
