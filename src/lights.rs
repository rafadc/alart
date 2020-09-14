use crate::colors::Color;
use crate::tuples::Tuple;

#[derive(PartialEq, Clone, Debug)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color
}


#[cfg(test)]
mod tests {

}