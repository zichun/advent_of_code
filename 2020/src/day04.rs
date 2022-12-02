use regex::Regex;
use std::collections::HashMap;

struct BadPassport
{
    kvp: HashMap<String, String>,
}

impl BadPassport
{
    fn parse(passport_str: &str) -> Self
    {
        let mut kvp = HashMap::new();
        passport_str
            .split(|x: char| x.is_whitespace())
            .for_each(|x| {
                if x.contains(":")
                {
                    let mut pat = x.split(":");
                    kvp.insert(pat.next().unwrap().to_string(),
                               pat.next().unwrap().to_string());
                }
            });
        BadPassport { kvp }
    }

    fn is_valid(&self) -> bool
    {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&key|
                 self.kvp.contains_key(key))
    }
}

#[derive(Debug)]
enum Height {
    Inch(u8),
    Cm(u8)
}
#[derive(Debug)]
enum EyeColor
{
    Amber, Blue, Brown, Grey, Green, Hazel, Other
}
struct Passport
{
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<EyeColor>,
    pid: Option<String>,
    cid: Option<String>
}

fn parse_year(value: &str, min_year: u16, max_year: u16) -> Option<u16>
{
    let year = value.parse::<u16>().ok()?;
    if year < min_year || year > max_year
    {
        None
    }
    else
    {
        Some(year)
    }
}

fn parse_height(value: &str) -> Option<Height>
{
    if value.ends_with("in")
    {
        let val = std::str::from_utf8(&value.as_bytes()[0..(value.len() - 2)]).ok()?.parse::<u8>().ok()?;
        if val < 59 || val > 76 {
            None
        } else {
            Some(Height::Inch(val))
        }
    }
    else if value.ends_with("cm")
    {
        let val = std::str::from_utf8(&value.as_bytes()[0..(value.len() - 2)]).ok()?.parse::<u8>().ok()?;
        if val < 150 || val > 193 {
            None
        } else {
            Some(Height::Cm(val))
        }
    }
    else
    {
        None
    }
}

fn parse_color(value: &str) -> Option<String>
{
    if value.len() == 7 && value.chars().next()? == '#' && value.chars().skip(1).all(|x| x.is_ascii_hexdigit()) {
        Some(value.to_string())
    } else {
        None
    }

}

fn parse_eye_color(value: &str) -> Option<EyeColor>
{
    match value {
        "amb" => Some(EyeColor::Amber),
        "blu" => Some(EyeColor::Blue),
        "brn" => Some(EyeColor::Brown),
        "gry" => Some(EyeColor::Grey),
        "grn" => Some(EyeColor::Green),
        "hzl" => Some(EyeColor::Hazel),
        "oth" => Some(EyeColor::Other),
        _ => None
    }
}

fn parse_passport_id(value: &str) -> Option<String>
{
    if value.len() == 9 && value.chars().all(|x| x.is_numeric()) {
        Some(value.to_string())
    } else {
        None
    }
}

impl Passport {
    fn parse(passport_str: &str) -> Self
    {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        passport_str
            .split(|x: char| x.is_whitespace())
            .for_each(|x| {
                if x.contains(":")
                {
                    let mut pat = x.split(":");
                    let key = pat.next().unwrap().to_string();
                    let value = pat.next().unwrap().to_string();

                    match key.trim() {
                        "byr" => byr = parse_year(&value, 1920, 2002),
                        "iyr" => iyr = parse_year(&value, 2010, 2020),
                        "eyr" => eyr = parse_year(&value, 2020, 2030),
                        "hgt" => hgt = parse_height(&value),
                        "hcl" => hcl = parse_color(&value),
                        "ecl" => ecl = parse_eye_color(&value),
                        "pid" => pid = parse_passport_id(&value),
                        "cid" => cid = Some(value.to_string()),
                        _ => ()
                    }
                }
            });

        Passport { byr, iyr, eyr, hgt, hcl, ecl, pid, cid }
    }

    fn is_valid(&self) -> bool
    {
        self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }
}

pub fn day04_1(inp: &str) -> usize {
    inp.split("\n\n")
        .map(|x| {
            x.split("\r\n\r\n").filter(|&y| BadPassport::parse(y).is_valid()).count()
        })
        .sum()
}

pub fn day04_2(inp: &str) -> usize {
    inp.split("\n\n")
        .map(|x| {
            x.split("\r\n\r\n").filter(|&y| Passport::parse(y).is_valid() ).count()
        })
        .sum()
}

#[test]
fn test_parse_bad_passport()
{
    let passport = BadPassport::parse("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm");
    assert_eq!(passport.is_valid(), true);
}

#[test]
fn test_day04_1() {
    let inp = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
    assert_eq!(day04_1(inp), 2);
}

#[test]
fn test_day04_2() {
    let inp = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:59in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2020 ecl:oth cid:129 byr:1920
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:150cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:000000001
"#;
    assert_eq!(day04_2(inp), 4);
}
