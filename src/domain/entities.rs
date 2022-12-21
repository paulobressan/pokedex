#[derive(PartialEq, Clone, PartialOrd, Ord, Eq)]
pub struct PokemonNumber(u16);
impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 0 && value < 899 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}
impl From<PokemonNumber> for u16 {
    fn from(value: PokemonNumber) -> Self {
        value.0
    }
}

#[derive(Clone)]
pub struct PokemonName(String);
impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}
impl From<PokemonName> for String {
    fn from(value: PokemonName) -> Self {
        value.0
    }
}

#[derive(Clone)]
enum PokemonType {
    Electric,
    Fire,
}
impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Electric" => Ok(PokemonType::Electric),
            "Fire" => Ok(PokemonType::Fire),
            _ => Err(()),
        }
    }
}
impl From<PokemonType> for String {
    fn from(value: PokemonType) -> Self {
        String::from(match value {
            PokemonType::Electric => "Electric",
            PokemonType::Fire => "Fire",
        })
    }
}

#[derive(Clone)]
pub struct PokemonTypes(Vec<PokemonType>);
impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in value.iter() {
                match PokemonType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}
impl From<PokemonTypes> for Vec<String> {
    fn from(pts: PokemonTypes) -> Self {
        let mut ts = vec![];
        for pt in pts.0.into_iter() {
            ts.push(String::from(pt))
        }
        ts
    }
}

#[derive(Clone)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}

#[cfg(test)]
impl PokemonNumber {
    pub fn pikachu() -> Self {
        Self(25)
    }

    pub fn charmander() -> Self {
        Self(4)
    }

    pub fn bad() -> Self {
        Self(0)
    }
}

#[cfg(test)]
impl PokemonName {
    pub fn pikachu() -> Self {
        Self(String::from("Pikachu"))
    }

    pub fn charmander() -> Self {
        Self(String::from("Charmander"))
    }

    pub fn bad() -> Self {
        Self(String::from(""))
    }
}

#[cfg(test)]
impl PokemonTypes {
    pub fn pikachu() -> Self {
        Self(vec![PokemonType::Electric])
    }

    pub fn charmander() -> Self {
        Self(vec![PokemonType::Fire])
    }
}
