use rand::Rng;

static PHONETICS: &'static [&str] = &[
  "AE", "AI", "AU",
  "EA", "EE", "EI", "EU",
  "IA", "IE", "IO","IU",
  "OA", "OE", "OI", "OU",
  "UA", "UE", "UI", "UO",
  "BA", "BE", "BI", "BO", "BU",
  "CA", "CE", "CI", "CO", "CU",
  "DA", "DE", "DI", "DO", "DU",
  "FA", "FE", "FI", "FO", "FU",
  "GA", "GE", "GI", "GO", "GU",
  "HA", "HE", "HI", "HO", "HU",
  "JA", "JE", "JI", "JO", "JU",
  "KA", "KE", "KI", "KO", "KU",
  "LA", "LE", "LI", "LO", "LU",
  "MA", "ME", "MI", "MO", "MU",
  "NA", "NE", "NI", "NO", "NU",
  "PA", "PE", "PI", "PO", "PU",
  "RA", "RE", "RI", "RO", "RU",
  "SA", "SE", "SI", "SO", "SU",
  "TA", "TE", "TI", "TO", "TU",
  "VA", "VE", "VI", "VO", "VU",
  "WA", "WE", "WI", "WO", "WU",
  "XA", "XE", "XI", "XO", "XU",
  "YA", "YE", "YI", "YO", "YU",
  "ZA", "ZE", "ZI", "ZO", "ZU",
];

static END_VOWELS: &'static [&str] = &[
  "A", "E", "I", "O", "U",
];

static END_CONSONANTS: &'static [&str] = &[
  "B", "C", "D", "F", "G", "H", "J", "K", "L", "M",
  "N", "P", "R", "S", "T", "V", "W", "X", "Y", "Z",
];

static CAN_DOUBLE: &'static [&str] = &[
  "C", "D", "F", "G", "K", "L", "M",
  "N", "P", "S", "T", "Z",
];

static DEFAULT_MIN: i32 = 1;
static DEFAULT_MAX: i32 = 5;

#[derive(Debug, Default)]
pub struct NameGenerationParams {
  pub min: Option<i32>,
  pub max: Option<i32>,
  pub can_have_end_letter: Option<bool>,
  pub must_have_end_letter: Option<bool>,
  pub can_have_numeral: Option<bool>,
  pub must_have_numeral: Option<bool>,
}

pub fn n_to_roman(n: i32) -> String {
  roman::to(n).unwrap_or("".to_string())
}

pub fn generate_name(params: NameGenerationParams) -> String {
  let mut name = String::new();
  let mut rng = rand::thread_rng();
  let mut length = rng.gen_range(params.min.unwrap_or(DEFAULT_MIN)..params.max.unwrap_or(DEFAULT_MAX));
  let mut first_iter = true;

  let numeral = if params.can_have_numeral.unwrap_or(true) && maybe() || params.must_have_numeral.unwrap_or(false) {
    Some(rand::random::<i32>() % 20)
  } else {
    None
  };

  while length > 0 {
    let mut phonetic = PHONETICS[rand::random::<usize>() % PHONETICS.len()].to_string();
    // Can only do this if not at the beginning
    let double_consonant = !first_iter && maybe_chance(4);
    
    if double_consonant && !phonetic.starts_with(&['A', 'E', 'I', 'O', 'U'][..]) {
      let first_char = phonetic.chars().next().unwrap();

      if CAN_DOUBLE.iter().any(|&c| c == first_char.to_string()) {
        phonetic = format!("{}{}", first_char, phonetic);
      }
    }

    name.push_str(phonetic.as_str());

    length -= 1;

    if first_iter {
      first_iter = false;
    }
  }

  let end_letter = params.can_have_end_letter.unwrap_or(true) && maybe_chance(4) || params.must_have_end_letter.unwrap_or(false);

  if end_letter {
    // If the last existing letter is a consonant, add a vowel, or vice versa
    let last_char = name.chars().last().unwrap();

    let end = if END_CONSONANTS.iter().any(|&c| c == last_char.to_string()) {
      END_VOWELS[rand::random::<usize>() % END_VOWELS.len()]
    } else {
      END_CONSONANTS[rand::random::<usize>() % END_CONSONANTS.len()]
    };

    name.push_str(end);
  }

  if let Some(numeral) = numeral {
    name.push_str(format!(" {}", &n_to_roman(numeral)).as_str());
  }

  name
}

pub fn maybe() -> bool {
  rand::random::<i32>() % 2 == 0
}

pub fn maybe_chance(chance: i32) -> bool {
  rand::random::<i32>() % chance == 0
}