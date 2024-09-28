//! Нагрузка на корпус судна
use std::rc::Rc;

use crate::data::{tree::DataTree, Path};

/// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
#[derive(Clone)]
pub struct Mass {
    mass: Path,
    loads_mass: Path,
    data: Rc<DataTree>,
}
///
impl Mass {
    ///
    pub fn new(
        mass: Path,
        loads_mass: Path,
        data: Rc<DataTree>,
    ) -> Self {
        Self {
            mass,            
            loads_mass,
            data,
        }
    }
}
///
impl IMass for Mass {
    /// Суммарная масса
    fn sum(&self) -> Option<f64> {
        let loads_mass = self.data.get(&self.loads_mass);
        if let Some(mass) = loads_mass {
            self.data.set(&self.mass, mass);
        } else {
            self.data.remove(&self.mass);
        }
        loads_mass
    }
}

#[doc(hidden)]
pub trait IMass {
    /// Суммарная масса
    fn sum(&self) -> Option<f64>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeMass {
    sum: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMass {
    pub fn new(sum: f64,) -> Self {
        Self { sum }
    }
}
#[doc(hidden)]
impl IMass for FakeMass {
    fn sum(&self) -> Option<f64> {
        Some(self.sum)
    }
}
