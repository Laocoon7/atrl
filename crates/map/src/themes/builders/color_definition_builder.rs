use crate::prelude::*;

#[derive(Default)]
pub struct ColorDefinitionBuilder {
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
    pub a: Option<u8>,
}

impl ColorDefinitionBuilder {
    /// Creates a new `ColorDefinitionBuilder`
    /// The following `.set_XXX` **must** be called prior to `.build()`:
    /// `.set_r()`
    /// `.set_g()`
    /// `.set_b()`
    /// `.set_a()`
    /// or
    /// `.set_rgb()`
    /// or
    /// `.set_rgba()`
    pub fn new() -> Self { Self { r: None, g: None, b: None, a: None } }

    pub fn set_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.r = Some(r);
        self.g = Some(g);
        self.b = Some(b);
        self.a = Some(255);
        self
    }

    pub fn set_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.r = Some(r);
        self.g = Some(g);
        self.b = Some(b);
        self.a = Some(a);
        self
    }

    pub fn set_r(mut self, r: u8) -> Self {
        self.r = Some(r);
        self
    }

    pub fn set_g(mut self, g: u8) -> Self {
        self.g = Some(g);
        self
    }

    pub fn set_b(mut self, b: u8) -> Self {
        self.b = Some(b);
        self
    }

    pub fn set_a(mut self, a: u8) -> Self {
        self.a = Some(a);
        self
    }

    pub fn build(self) -> Result<ColorDefinition> {
        let r = match self.r {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("ColorDefinition".to_string(), "r".to_string()))
            }
        };

        let g = match self.g {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("ColorDefinition".to_string(), "g".to_string()))
            }
        };

        let b = match self.b {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("ColorDefinition".to_string(), "b".to_string()))
            }
        };

        let a = match self.a {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("ColorDefinition".to_string(), "a".to_string()))
            }
        };

        Ok(ColorDefinition { r, g, b, a })
    }
}
