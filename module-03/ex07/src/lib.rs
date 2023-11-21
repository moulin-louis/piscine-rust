mod test;
use std::option::Option;
struct EncodingError;
struct DecodingError;

trait Field: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(field: &str) -> Result<Self, DecodingError>;
}

impl Field for String {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
        for x in self.chars() {
            if x == ',' || x == '\n' {
                return Err(EncodingError);
            }
        }
        *target = self.clone();
        Ok(())
    }

    fn decode(field: &str) -> Result<Self, DecodingError> {
        Ok(field.to_string())
    }
}

impl<T: Field> Field for Option<T> {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
        match self {
            Some(val) => val.encode(target),
            None => Err(EncodingError),
        }
    }

    fn decode(field: &str) -> Result<Self, DecodingError> {
        if field.is_empty() {
            return Err(DecodingError);
        }
        T::decode(field).map(Some)
    }
}

#[macro_export]
macro_rules! impl_field_for_int {
    ($a:expr) => {
        impl Field for $a {
            fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
                match self {
                    Some(val) => val.encode(target),
                    None => Err(EncodingError),
                }
            }

            fn decode(field: &str) -> Result<Self, DecodingError> {
                if field.is_empty() {
                    return Err(DecodingError);
                }
                T::decode(field).map(Some)
            }
        }
    };
}


trait Record {}