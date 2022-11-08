use std::fmt::{self, Debug};

use noise::{NoiseFn, Perlin};
use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};

pub struct Noise {
    seed: u32,
    perlin: Perlin,
}

#[allow(dead_code)]
impl Noise {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        Self { seed, perlin }
    }

    pub fn get<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(&mut self, x: X, y: Y, z: Z) -> f64 {
        let x = x.into();
        let y = y.into();
        let z = z.into();

        self.perlin.get([x, y, z])
    }
}

impl Serialize for Noise {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Noise", 1)?;
        state.serialize_field("seed", &self.seed)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Noise {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Seed,
        }

        struct NoiseVisitor;

        impl<'de> Visitor<'de> for NoiseVisitor {
            type Value = Noise;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Noise")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let seed =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Noise::new(seed))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut seed = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Seed => {
                            if seed.is_some() {
                                return Err(de::Error::duplicate_field("seed"));
                            }
                            seed = Some(map.next_value()?);
                        }
                    }
                }
                let seed = seed.ok_or_else(|| de::Error::missing_field("seed"))?;
                Ok(Noise::new(seed))
            }
        }

        const FIELDS: &[&str] = &["seed"];
        deserializer.deserialize_struct("Noise", FIELDS, NoiseVisitor)
    }
}

impl Debug for Noise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Noise {{ seed:{} }}", self.seed))
    }
}
